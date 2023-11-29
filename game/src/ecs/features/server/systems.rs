use bevy::{log, prelude::*};
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::ecs::{
    features::server::messages::GetServerMessage,
    plugins::websocket::{WsConnections, WsEvent, WsServer},
};

pub fn startup_socket_listener(
    tokio_runtime: Res<TokioTasksRuntime>,
    mut server: ResMut<WsServer>,
) {
    server.start_listening(&tokio_runtime, "localhost:8081");
}

pub fn log_connection_events(mut event_reader: EventReader<WsEvent>) {
    for event in event_reader.read() {
        match event {
            WsEvent::Connected { connection } => {
                log::info!("client connected: {}", connection.id);
            }

            WsEvent::Disconnected { connection_id } => {
                log::info!("client disconnected: {}", connection_id);
            }

            _ => {}
        }
    }
}

pub fn send_clients_connected_on_join(
    mut event_reader: EventReader<WsEvent>,
    connections: Res<WsConnections>,
) {
    for event in event_reader.read() {
        if let WsEvent::Connected { connection } = event {
            connection
                .send(GetServerMessage {
                    clients_connected: connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
                })
                .ok();
        }
    }
}

// pub fn handle_message(mut event_reader: EventReader<WsEvent>, connections: Res<WsConnections>) {
//     for event in event_reader.read() {
//         if let WsEvent::Message {
//             connection_id,
//             message,
//         } = event
//         {
//             let connection = connections.get(connection_id).unwrap();

//             match message {
//                 WsMessage::Text(data) => {
//                     log::info!("text message: {}", &data);
//                 }

//                 WsMessage::Binary(data) => {
//                     let message: Message<'_> = flatbuffers::root::<Message>(data).unwrap();
//                     log::info!("binary message: {:?}", message.message_type());
//                     match message.message_type() {
//                         MessageType::RequestGetServer => {
//                             connection
//                                 .send(GetServerMessage {
//                                     clients_connected: connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
//                                 })
//                                 .ok();
//                         }

//                         MessageType::NoOp => {
//                             connection
//                                 .send_raw(WsMessage::Text("Welcome to ST-RT-API".to_string()))
//                                 .ok();
//                         }

//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }
// }
