use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use bevy::prelude::*;
use bevy::utils::Uuid;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener};

use crate::ecs::components::client::ClientComponent;
use crate::generated::message::{Message, MessageType};
use crate::messages::server_stat_event;

pub fn startup_socket_listener(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8080");
}

pub fn assign_client_to_socket(
    mut commands: Commands,
    connections: Query<(Entity, &WsConnection), Added<WsConnection>>,
) {
    for (entity, conn) in connections.iter() {
        let uuid = Uuid::new_v4();
        let name = format!("guest-{}", uuid.as_hyphenated());
        commands.entity(entity).insert(ClientComponent {
            uuid: uuid.as_u128(),
            name,
            energy: 10,
            energy_capacity: 10,
            energy_generation_sec: 1,
            unit_capacity: 10,
        });
        conn.send(WSMessage::binary(server_stat_event::buffer(
            connections.iter().count() as u16,
        )));
        println!("Client connected: {}", uuid.as_hyphenated());
    }
}

pub fn receive_message(
    mut commands: Commands,
    connections: Query<(Entity, &WsConnection, &ClientComponent)>,
) {
    for (entity, conn, client) in connections.iter() {
        //TODO: lets use par_iter() here and send events for each type of message
        loop {
            match conn.receive() {
                Ok(message) => {
                    println!("Energy {}", client.energy);
                    match message {
                        WSMessage::Text(data) => {
                            println!("Text Message: {}", &data)
                        }
                        WSMessage::Binary(data) => {
                            let message = flatbuffers::root::<Message>(&data).unwrap();
                            match message.message_type() {
                                MessageType::RequestGetServer => {
                                    conn.send(WSMessage::binary(server_stat_event::buffer(
                                        connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
                                    )));
                                }
                                MessageType::NoOp => {
                                    conn.send(WSMessage::text("Welcome to ST-RT-API"));
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}
