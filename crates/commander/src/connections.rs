use std::sync::Arc;

use bevy::{
    ecs::{
        event::EventReader,
        system::{ResMut, Resource},
    },
    prelude::Deref,
    utils::HashMap,
};

use super::{
    connection::{ConnectionId, SocketConnection},
    event::SocketConnectionEvent,
};

#[derive(Resource, Deref, Default)]
pub struct SocketConnections {
    map: HashMap<ConnectionId, Arc<SocketConnection>>,
}

impl SocketConnections {
    fn on_connected(&mut self, connection: Arc<SocketConnection>) {
        self.map.insert(connection.id, connection);
    }

    fn on_disconnected(&mut self, connection_id: &ConnectionId) {
        self.map.remove(connection_id);
    }
}

pub(crate) fn update_connections_map(
    mut event_reader: EventReader<SocketConnectionEvent>,
    mut connections: ResMut<SocketConnections>,
) {
    for event in event_reader.read() {
        match event {
            SocketConnectionEvent::Connected { connection } => {
                connections.on_connected(Arc::clone(connection));
            }

            SocketConnectionEvent::Disconnected { connection_id } => {
                connections.on_disconnected(connection_id);
            }
        }
    }
}
