use std::sync::Arc;

use bevy::{log, tasks::block_on};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::ecs::plugins::websocket::WsMessage;

use super::{ConnectionId, WsEvent, WsMessageType};

#[derive(Debug)]
pub struct WsConnection {
    pub id: ConnectionId,

    sender: async_channel::Sender<WsMessage>,
}

impl WsConnection {
    pub(super) async fn accept(events_tx: async_channel::Sender<WsEvent>, stream: TcpStream) {
        let addr = stream
            .peer_addr()
            .expect("connected streams should have a peer address");

        log::debug!("new connection from {}", addr);

        let stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("websocket handshake failed");

        let (write_tx, write_rx) = async_channel::unbounded::<WsMessage>();

        let connection_id = ConnectionId::new();

        let connection = Arc::new(WsConnection {
            id: connection_id,

            sender: write_tx,
        });

        // If this fails then the server was dropped and there is no need
        // to accept the websocket.
        if events_tx
            .send(WsEvent::Connected { connection })
            .await
            .is_err()
        {
            log::debug!("websocket server was dropped");
            return;
        }

        WsConnectionListener {
            connection_id,

            stream: Some(stream),

            write_rx,
            events_tx,
        }
        .listen()
        .await;
    }

    pub fn send_raw(&self, message: WsMessage) -> Result<(), SendError> {
        self.sender.try_send(message).map_err(|_| SendError)
    }

    pub fn send<T>(&self, message: T) -> Result<(), SendError>
    where
        T: WsMessageType,
    {
        self.sender
            .try_send(message.to_message())
            .map_err(|_| SendError)
    }
}

#[derive(Debug)]
pub struct SendError;

struct WsConnectionListener {
    connection_id: ConnectionId,

    stream: Option<WebSocketStream<TcpStream>>,

    write_rx: async_channel::Receiver<WsMessage>,
    events_tx: async_channel::Sender<WsEvent>,
}

impl WsConnectionListener {
    async fn listen(&mut self) {
        let (mut write, mut read) = self
            .stream
            .take()
            .expect("listen has already been called")
            .split();

        loop {
            tokio::select! {
                message = self.write_rx.recv() => {
                    if let Ok(message) = message {
                        let _ = write
                            .send(match message {
                                WsMessage::Text(data) => Message::Text(data),
                                WsMessage::Binary(data) => Message::Binary(data),
                            })
                            .await;
                    } else {
                        break;
                    }
                },

                message = read.next() => {
                    if let Some(Ok(message)) = message {
                        let message = match message {
                            Message::Text(data) => {
                                WsMessage::Text(data)
                            }

                            Message::Binary(data) => {
                                WsMessage::Binary(data)
                            }

                            _ => {
                                // Add other message types here if you want to support them.
                                continue;
                            }
                        };

                        let _ = self
                            .events_tx
                            .send(WsEvent::Message {
                                connection_id: self.connection_id,
                                message,
                            })
                            .await;
                    } else {
                        break;
                    }
                },
            }
        }
    }
}

impl Drop for WsConnectionListener {
    fn drop(&mut self) {
        // When the connection is dropped, we need to make sure to notify over the
        // event channel so proper cleanup can be done.
        let _ = block_on(self.events_tx.send(WsEvent::Disconnected {
            connection_id: self.connection_id,
        }));
    }
}
