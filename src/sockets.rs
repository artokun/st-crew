use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use bevy::prelude::*;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};
extern crate flatbuffers;

#[allow(dead_code, unused_imports)]
#[path = "generated/message_generated.rs"]
mod message_generated;
pub use message_generated::message::{
    Message, MessageArgs, MessageType, ServerStatEvent, ServerStatEventArgs,
};

use self::message_generated::message;

fn message_stats_buffer(clients_connected: u32) -> WSMessage {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
    let event = ServerStatEvent::create(&mut builder, &ServerStatEventArgs { clients_connected });
    let message = Message::create(
        &mut builder,
        &MessageArgs {
            message_type: MessageType::ServerStatEvent,
            message: Some(event.as_union_value()),
            ..Default::default()
        },
    );
    builder.finish(message, None);
    WSMessage::Binary(builder.finished_data().to_vec())
}

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
                                conn.send(message_stats_buffer(connections.iter().count() as u32));
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
