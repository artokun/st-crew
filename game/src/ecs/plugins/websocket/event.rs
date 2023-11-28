use std::sync::Arc;

use bevy::ecs::event::Event;

use super::{ConnectionId, WsConnection};

#[derive(Debug, Event)]
pub enum WsEvent {
    Connected {
        connection: Arc<WsConnection>,
    },

    Disconnected {
        connection_id: ConnectionId,
    },

    Message {
        connection_id: ConnectionId,
        message: WsMessage,
    },
}

#[derive(Debug)]
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
}
