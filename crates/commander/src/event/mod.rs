use std::sync::Arc;

use bevy::ecs::event::Event;
use serde::Serialize;
use utoipa::ToSchema;

use super::connection::{ConnectionId, SocketConnection};

pub(crate) mod payload;
pub(crate) mod systems;

pub trait SocketEvent: ToSchema<'static> + Serialize + Send + Sync + 'static {
    const NAME: &'static str;
}

#[derive(Debug, Event)]
pub enum SocketConnectionEvent {
    Connected { connection: Arc<SocketConnection> },
    Disconnected { connection_id: ConnectionId },
}
