use bevy::utils::Uuid;
use bevy::{log, prelude::*};
use bevy_tokio_tasks::TokioTasksRuntime;

use crate::ecs::components::client::ClientComponent;
use crate::ecs::plugins::websocket::{WsConnections, WsEvent, WsMessage, WsServer};
use crate::generated::message::{Message, MessageType};
use crate::messages::server_stat_event::GetServerMessage;

use super::ConnectedPlayers;

pub fn startup_socket_listener(
    tokio_runtime: Res<TokioTasksRuntime>,
    mut server: ResMut<WsServer>,
) {
    server.start_listening(&tokio_runtime, "localhost:8081");
}

pub fn update_connected_players(
    mut commands: Commands,
    mut event_reader: EventReader<WsEvent>,
    connections: Res<WsConnections>,
    mut connected_players: ResMut<ConnectedPlayers>,
) {
    for event in event_reader.read() {
        match event {
            WsEvent::Connected { connection } => {
                println!("Client connected: {}", connection.id);

                let uuid = Uuid::new_v4();

                let entity = commands
                    .spawn(ClientComponent {
                        uuid,
                        name: format!("guest-{}", uuid.as_hyphenated()),
                        energy: 10,
                        energy_capacity: 10,
                        energy_generation_sec: 1,
                        unit_capacity: 10,
                    })
                    .id();

                connected_players.on_player_connected(connection.id, entity);

                connection.send(GetServerMessage {
                    clients_connected: connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
                });
            }

            WsEvent::Disconnected { connection_id } => {
                println!("Client disconnected: {}", connection_id);

                match connected_players.on_player_disconnected(connection_id) {
                    Some(entity) => {
                        commands.entity(entity).despawn();
                    }

                    None => {
                        log::warn!("No entity found for connection id: {}", connection_id);
                    }
                }
            }

            _ => {}
        }
    }
}

pub fn handle_message(mut event_reader: EventReader<WsEvent>, connections: Res<WsConnections>) {
    for event in event_reader.read() {
        if let WsEvent::Message {
            connection_id,
            message,
        } = event
        {
            println!("Client message: {}", connection_id);

            let connection = connections.get(connection_id).unwrap();

            match message {
                WsMessage::Text(data) => {
                    println!("Text Message: {}", &data)
                }

                WsMessage::Binary(data) => {
                    let message = flatbuffers::root::<Message>(data).unwrap();

                    match message.message_type() {
                        MessageType::RequestGetServer => {
                            connection.send(GetServerMessage {
                                clients_connected: connections.iter().count() as u16, //TODO: lets create a game state resource and use that for connection counts
                            });
                        }

                        MessageType::NoOp => {
                            connection
                                .send_raw(WsMessage::Text("Welcome to ST-RT-API".to_string()));
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
