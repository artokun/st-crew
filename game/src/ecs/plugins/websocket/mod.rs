use bevy::prelude::*;

mod connection;
mod connection_id;
mod connections;
mod event;
mod message;
#[cfg(test)]
mod mock;
mod server;

pub use connection::*;
pub use connection_id::*;
pub use connections::*;
pub use event::*;
pub use message::*;
pub use server::WsServer;

use self::server::{write_events_from_channel, WsEventQueue};

// TODO: lets add the connection state resource here as well as define the message event types,
// lets also create a fixed time step scheduler to handle the energy generation

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        let (events_tx, events_rx) = async_channel::unbounded();

        app.insert_resource(WsServer::new(events_tx))
            .insert_resource(WsEventQueue::new(events_rx))
            .insert_resource(WsConnections::default())
            .add_event::<WsEvent>()
            .add_systems(
                PreUpdate,
                (write_events_from_channel, update_connections_map).chain(),
            );
    }
}

#[cfg(test)]
mod tests {
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::tungstenite::Message;

    use crate::ecs::plugins::websocket::{WsEvent, WsMessage, WsServer};

    // #[tokio::test]
    // async fn can_connect() {
    //     let (server_tx, server_rx) = async_channel::unbounded();

    //     let server = WsServer::new(server_tx).mock().await;

    //     let (mut socket, _) = tokio_tungstenite::connect_async(server.uri())
    //         .await
    //         .expect("failed to connect");

    //     socket
    //         .send(Message::Text("Hello WebSocket".into()))
    //         .await
    //         .unwrap();

    //     let message = server_rx.try_recv().expect("no message received");

    //     println!("Received: {:?}", message);

    //     loop {
    //         let msg = socket
    //             .next()
    //             .await
    //             .expect("error reading message")
    //             .expect("error reading message");
    //         println!("Received: {}", msg);
    //     }
    // }

    #[tokio::test]
    async fn websocket_connection_lifecycle() {
        let (server_tx, server_rx) = async_channel::unbounded();

        let server = WsServer::new(server_tx).mock().await;

        let (mut client_socket, _) = tokio_tungstenite::connect_async(server.uri())
            .await
            .expect("failed to connect");

        let connection = match server_rx.recv().await.expect("no message received") {
            WsEvent::Connected { connection } => connection,
            other => panic!("did not receive a connected event: {:?}", other),
        };

        let connection_id = connection.id;

        client_socket
            .send(Message::Text("foo".to_string()))
            .await
            .expect("failed to send message");

        match server_rx.recv().await.expect("no message received") {
            WsEvent::Message {
                message: WsMessage::Text(message),
                ..
            } => assert_eq!(message, "foo"),

            other => panic!("did not receive a message event: {:?}", other),
        }

        connection
            .send_raw(WsMessage::Text("bar".to_string()))
            .expect("failed to send message");

        match client_socket.next().await.expect("error reading message") {
            Ok(Message::Text(message)) => assert_eq!(message, "bar"),
            other => panic!("did not receive a message event: {:?}", other),
        }

        drop(connection);

        match server_rx.recv().await.expect("no message received") {
            WsEvent::Disconnected {
                connection_id: event_connection_id,
            } => assert_eq!(connection_id, event_connection_id),

            other => panic!("did not receive a disconnect event: {:?}", other),
        }
    }
}
