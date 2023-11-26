use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use bevy::prelude::*;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};
use crate::generated::message_generated::message::{ self, MessageType };
use crate::messages::server_stat_event;

fn startup_socket_listener(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8080");
}

fn receive_message(mut commands: Commands, connections: Query<(Entity, &WsConnection)>) {
    for (entity, conn) in connections.iter() {
        loop {
            match conn.receive() {
                Ok(message) => match message {
                    WSMessage::Binary(data) => {
                        let message = message::root_as_message(&data).unwrap();
                        match message.message_type() {
                            MessageType::RequestServerStatEvent => {
                                conn.send(server_stat_event::buffer(connections.iter().count() as u32));
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
