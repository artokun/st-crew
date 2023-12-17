use std::collections::HashMap;

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{de::IgnoredAny, Deserialize};
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::{client::IntoClientRequest, http::Uri, Message};

use crate::{commands::RpcCommand, models::ApiError};

use self::{
    event::SocketEventMessage,
    rpc::{RpcCall, RpcReply},
};

mod event;
mod rpc;

type Websocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct WebsocketSdk {
    events_rx: async_channel::Receiver<()>,

    write_tx: mpsc::UnboundedSender<SocketMessage>,

    next_rpc_id: u64,
}

impl WebsocketSdk {
    pub async fn new(uri: Uri) -> Result<Self, tungstenite::Error> {
        let mut request = uri.into_client_request()?;

        request
            .headers_mut()
            .insert("Accept", "application/msgpack; named=false".parse()?);

        let (socket, _) = tokio_tungstenite::connect_async(request).await?;

        Ok(Self::with_socket(socket))
    }

    fn with_socket(socket: Websocket) -> Self {
        let (events_tx, events_rx) = async_channel::unbounded();
        let (write_tx, write_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let (write, read) = socket.split();

            SocketHandler {
                write,
                read,

                events: events_tx,
                write_rx,

                rpc: HashMap::new(),
            }
            .listen()
            .await
        });

        Self {
            events_rx,

            write_tx,

            next_rpc_id: 0,
        }
    }

    pub async fn execute<C>(&mut self, command: C) -> Result<C::Output, ExecuteError>
    where
        C: RpcCommand,
    {
        let rpc_id = self.next_rpc_id;

        self.next_rpc_id += 1;

        let (response_tx, response_rx) = oneshot::channel();

        let message = rmp_serde::to_vec_named(&RpcCall {
            id: rpc_id,
            command: C::NAME,
            input: command,
        })?;

        self.write_tx
            .send(SocketMessage::Command {
                id: rpc_id,
                message,
                response_tx,
            })
            .map_err(|_| ExecuteError::Closed)?;

        let data = response_rx.await.map_err(|_| ExecuteError::Closed)?;

        let rpc_reply = rmp_serde::decode::from_slice::<RpcReply<C::Output>>(&data)?;

        Ok(rpc_reply.output)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ExecuteError {
    #[error("failed to serialize command")]
    Serialize(#[from] rmp_serde::encode::Error),

    #[error("failed to deserialize response")]
    Deserialize(#[from] rmp_serde::decode::Error),

    #[error("websocket was closed")]
    Closed,
}

enum SocketMessage {
    Text(String),
    Binary(Vec<u8>),

    Command {
        id: u64,
        message: Vec<u8>,
        response_tx: oneshot::Sender<Vec<u8>>,
    },
}

struct SocketHandler {
    write: SplitSink<Websocket, Message>,
    read: SplitStream<Websocket>,

    events: async_channel::Sender<()>,

    write_rx: mpsc::UnboundedReceiver<SocketMessage>,

    rpc: HashMap<u64, oneshot::Sender<Vec<u8>>>,
}

impl SocketHandler {
    async fn listen(mut self) {
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

    async fn on_write(&mut self, message: SocketMessage) {
        match message {
            SocketMessage::Text(message) => {
                let _ = self.write.send(Message::Text(message)).await;
            }

            SocketMessage::Binary(message) => {
                let _ = self.write.send(Message::Binary(message)).await;
            }

            SocketMessage::Command {
                id,
                message,
                response_tx,
            } => {
                self.rpc.insert(id, response_tx);

                let _ = self.write.send(Message::Binary(message)).await;
            }
        }
    }

    async fn on_read(&mut self, message: Message) -> Result<(), ConnectionClosed> {
        let Message::Binary(bytes) = message else {
            return Ok(());
        };

        eprintln!("received message: {}", base64::encode(&bytes));

        let message: ServerMessage = match rmp_serde::from_slice(&bytes) {
            Ok(message) => message,

            Err(err) => {
                eprintln!("failed to deserialize message: {}", err);

                return Ok(());
            }
        };

        match message {
            ServerMessage::Event(event) => {
                // TODO: deserialize events

                println!("event: {:?}", event);
            }

            ServerMessage::Reply(reply) => {
                if let Some(response_tx) = self.rpc.remove(&reply.id) {
                    // We don't know how to deserialize this, so we just send the full message
                    let _ = response_tx.send(bytes);
                } else {
                    eprintln!("received reply for unknown rpc id: {}", reply.id);
                }
            }

            ServerMessage::Error(err) => {
                eprintln!("received error: {:?}", err);
            }
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("connection closed")]
struct ConnectionClosed;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ServerMessage {
    Event(SocketEventMessage<IgnoredAny>),
    Reply(RpcReply<IgnoredAny>),
    Error(ApiError),
}
