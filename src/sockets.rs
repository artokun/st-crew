use crate::generated::message::{self, MessageType};
use crate::messages::server_stat_event;
use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use bevy::prelude::*;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};

fn startup_socket_listener(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8080");
}

fn receive_message(mut commands: Commands, connections: Query<(Entity, &WsConnection)>) {
    for (entity, conn) in connections.iter() {
        loop {
            match conn.receive() {
                Ok(message) => match message {
                    WSMessage::Text(data) => {
                        println!("Text Message: {}", &data)
                    }
                    WSMessage::Binary(data) => {
                        let message = message::root_as_message(&data).unwrap();
                        match message.message_type() {
                            MessageType::RequestServerStatEvent => {
                                conn.send(WSMessage::binary(server_stat_event::buffer(
                                    connections.iter().count() as u32,
                                )));
                            }
                            MessageType::NoOpEvent => {
                                conn.send(WSMessage::text("Welcome to ST-RT-API"));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WsPlugin)
            .add_systems(Startup, startup_socket_listener)
            .add_systems(Update, receive_message);
    }
}
