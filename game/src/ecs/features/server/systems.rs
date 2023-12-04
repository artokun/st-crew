use bevy::{log, prelude::*, tasks::block_on};
use st_commander::{connections::SocketConnections, event::SocketConnectionEvent, CommanderServer};

use super::{models::ServerInfo, socket_events::ServerInfoSocketEvent};

pub fn startup_socket_listener(mut server: ResMut<CommanderServer>) {
    let addr = block_on(server.start_listening("127.0.0.1:8081"))
        .expect("failed to wait for server to be ready");

    log::info!("listening on {}", addr);
}

pub fn log_connection_events(mut event_reader: EventReader<SocketConnectionEvent>) {
    for event in event_reader.read() {
        match event {
            SocketConnectionEvent::Connected { connection } => {
                log::info!("client connected: {}", connection.id);
            }

            SocketConnectionEvent::Disconnected { connection_id } => {
                log::info!("client disconnected: {}", connection_id);
            }
        }
    }
}

pub fn send_clients_connected_on_join(
    mut event_reader: EventReader<SocketConnectionEvent>,
    connections: Res<SocketConnections>,
) {
    for event in event_reader.read() {
        if let SocketConnectionEvent::Connected { connection } = event {
            connection
                .send_event(ServerInfoSocketEvent(ServerInfo {
                    connected_clients: connections.iter().count(),
                }))
                .ok();
        }
    }
}

// pub fn handle_message(
//     mut event_reader: EventReader<WsEvent>,
//     connections: Res<WsConnections>,
//     players: Res<ConnectedPlayers>,
//     mut query: Query<&mut Energy>,
// ) {
//     for event in event_reader.read() {
//         if let WsEvent::Message {
//             connection_id,
//             message,
//         } = event
//         {
//             let connection = connections.get(connection_id).unwrap();
//             let player_entity = players.get(connection_id).unwrap().to_owned();
//             let mut energy = query.get_mut(player_entity).unwrap();

//             match message {
//                 WsMessage::Text(data) => {
//                     log::info!("text message: {}", &data);
//                 }

//                 WsMessage::Binary(data) => {
//                     let message: Message<'_> = flatbuffers::root::<Message>(data).unwrap();
//                     log::info!("RECEIVED: {:?}", message.message_type());
//                     match message.message_type() {
//                         MessageType::RequestGetServer => {
//                             match energy.spend(2) {
//                                 Ok(_) => {
//                                     connection
//                                         .send(GetServerMessage {
//                                             clients_connected: connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
//                                         })
//                                         .ok();
//                                 }

//                                 Err(err) => {
//                                     connection.send_raw(WsMessage::Text(err)).ok();
//                                 }
//                             }
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
