use std::{borrow::Cow, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use bevy::{log, tasks::block_on};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{de::IgnoredAny, Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    connection::ConnectionId,
    connection::SocketConnection,
    data_format::DataFormat,
    event::SocketConnectionEvent,
    response::ApiError,
    rpc::{DispatchError, RpcCall},
};

pub mod accept_headers;
mod schema;
mod state;

pub use schema::*;
pub use state::*;

#[derive(Deserialize)]
pub(crate) struct WebsocketParams {
    format: Option<WebsocketDataFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum WebsocketDataFormat {
    Json,
    MsgPack,
    #[serde(rename = "msgpack_unnamed")]
    MsgPackUnnamed,
    Cbor,
    #[serde(rename = "cbor_packed")]
    CborPacked,
    Form,
}

pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    query: Query<WebsocketParams>,
    data_format: Option<DataFormat>,
    Extension(state): Extension<CommanderState>,
) -> Response {
    let query_format = query.format.map(|format| match format {
        WebsocketDataFormat::Json => DataFormat::Json,
        WebsocketDataFormat::MsgPack => DataFormat::MsgPack { named: true },
        WebsocketDataFormat::MsgPackUnnamed => DataFormat::MsgPack { named: false },
        WebsocketDataFormat::Cbor => DataFormat::Cbor { packed: false },
        WebsocketDataFormat::CborPacked => DataFormat::Cbor { packed: true },
        WebsocketDataFormat::Form => DataFormat::Form,
    });

    let Some(data_format) = query_format.or(data_format) else {
        return (StatusCode::NOT_ACCEPTABLE).into_response();
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state.into_inner(), data_format))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<InnerCommanderState>,
    data_format: DataFormat,
) {
    let (write_tx, write_rx) = mpsc::unbounded_channel();

    let connection = Arc::new(SocketConnection::new(write_tx));

    let connection_id = connection.id;

    // If this fails then the server was dropped and there is no need
    // to accept the websocket.
    if state
        .events_tx
        .send(SocketConnectionEvent::Connected { connection })
        .await
        .is_err()
    {
        log::debug!("websocket server was dropped");
        return;
    }

    let (write, read) = socket.split();

    SocketConnectionListener {
        state,

        connection_id,
        data_format,

        write,
        read,

        write_rx,
    }
    .listen()
    .await;
}

struct SocketConnectionListener {
    state: Arc<InnerCommanderState>,

    connection_id: ConnectionId,
    data_format: DataFormat,

    write: SplitSink<WebSocket, Message>,
    read: SplitStream<WebSocket>,

    write_rx: mpsc::UnboundedReceiver<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

impl SocketConnectionListener {
    async fn listen(&mut self) {
        loop {
            tokio::select! {
                message = self.write_rx.recv() => {
                    if let Some(data) = message {
                        self.on_write(data).await
                    } else {
                        return;
                    }
                },

                message = self.read.next() => {
                    if let Some(Ok(message)) = message {
                        if self.on_read(message).await.is_err() {
                            return;
                        }
                    } else {
                        return;
                    }
                },
            };
        }
    }

    async fn on_write(&mut self, data: Box<dyn erased_serde::Serialize + Send + Sync>) {
        match self.data_format.serialize(&data) {
            Ok(data) => {
                let _ = self.write.send(Message::Binary(data)).await;
            }

            Err(err) => {
                log::error!("failed to serialize data: {}", err);
            }
        }
    }

    async fn on_read(&mut self, message: Message) -> Result<(), ConnectionClosed> {
        let Message::Binary(bytes) = message else {
            return Ok(());
        };

        if let Err(err) = self.handle_dispatch(bytes).await {
            if matches!(
                err,
                HandleDispatchError::Reply(SendError::ConnectionClosed(_))
            ) {
                return Err(ConnectionClosed);
            }

            if let Err(err) = self.send(&ApiError::from(err)).await {
                match err {
                    SendError::Serialize(err) => {
                        log::error!("failed to serialize error: {}", err);
                    }

                    SendError::ConnectionClosed(_) => {
                        return Err(ConnectionClosed);
                    }
                }
            }
        }

        Ok(())
    }

    async fn handle_dispatch(&mut self, bytes: Vec<u8>) -> Result<(), HandleDispatchError> {
        let rpc_call = self
            .data_format
            .deserialize::<RpcCall<IgnoredAny>>(&bytes)
            .map_err(HandleDispatchError::DeserializeCommand)?;

        let Some(command) = self.state.commands.get(rpc_call.command.as_ref()) else {
            return Err(HandleDispatchError::UnknownCommand(rpc_call.command));
        };

        self.send_raw(
            command
                .dispatch(rpc_call.id, self.data_format, &bytes)
                .await?,
        )
        .await
        .map_err(|err| HandleDispatchError::Reply(SendError::ConnectionClosed(err)))
    }

    async fn send<T>(&mut self, data: &T) -> Result<(), SendError>
    where
        T: serde::Serialize,
    {
        let reply_bytes = self
            .data_format
            .serialize(data)
            .map_err(SendError::Serialize)?;

        Ok(self.send_raw(reply_bytes).await?)
    }

    async fn send_raw(&mut self, bytes: Vec<u8>) -> Result<(), ConnectionClosed> {
        self.write
            .send(Message::Binary(bytes))
            .await
            .map_err(|_err| ConnectionClosed)
    }
}

impl Drop for SocketConnectionListener {
    fn drop(&mut self) {
        log::debug!("websocket connection closed");

        // When the connection is dropped, we need to make sure to notify over the
        // event channel so proper cleanup can be done.
        let _ = block_on(
            self.state
                .events_tx
                .send(SocketConnectionEvent::Disconnected {
                    connection_id: self.connection_id,
                }),
        );
    }
}

#[derive(Debug, thiserror::Error)]
#[error("connection closed")]
struct ConnectionClosed;

#[derive(Debug, thiserror::Error)]
enum SendError {
    #[error("failed to serialize reply: {0}")]
    Serialize(Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    ConnectionClosed(#[from] ConnectionClosed),
}

#[derive(Debug, thiserror::Error)]
enum HandleDispatchError {
    #[error("failed to deserialize command: {0}")]
    DeserializeCommand(Box<dyn std::error::Error + Send + Sync>),

    #[error("unknown command: {0}")]
    UnknownCommand(Cow<'static, str>),

    #[error("command dispatch failed: {0}")]
    Dispatch(#[from] DispatchError),

    #[error("failed to send reply: {0}")]
    Reply(#[from] SendError),
}

impl From<HandleDispatchError> for ApiError {
    fn from(err: HandleDispatchError) -> Self {
        match err {
            HandleDispatchError::DeserializeCommand(err) => ApiError::new(StatusCode::BAD_REQUEST)
                .with_name("invalid_body")
                .with_error(&err),

            err @ HandleDispatchError::UnknownCommand(_) => ApiError::new(StatusCode::BAD_REQUEST)
                .with_name("unknown_command")
                .with_error(err),

            HandleDispatchError::Dispatch(err) => ApiError::internal_server_error(err),
            HandleDispatchError::Reply(err) => ApiError::internal_server_error(err),
        }
    }
}
