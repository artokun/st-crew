use std::sync::Arc;

use bevy::{
    ecs::{
        event::EventReader,
        system::{ResMut, Resource},
    },
    prelude::Deref,
    utils::HashMap,
};

use super::{ConnectionId, WsConnection, WsEvent};

#[derive(Resource, Deref, Default)]
pub struct WsConnections {
    map: HashMap<ConnectionId, Arc<WsConnection>>,
}

impl WsConnections {
    pub(super) fn on_connected(&mut self, connection: Arc<WsConnection>) {
        self.map.insert(connection.id, connection);
    }

    pub(super) fn on_disconnected(&mut self, connection_id: &ConnectionId) {
        self.map.remove(connection_id);
    }
}

pub(super) fn update_connections_map(
    mut event_reader: EventReader<WsEvent>,
    mut connections: ResMut<WsConnections>,
) {
    for event in event_reader.read() {
        match event {
            WsEvent::Connected { connection } => {
                connections.on_connected(Arc::clone(connection));
            }

            WsEvent::Disconnected { connection_id } => {
                connections.on_disconnected(connection_id);
            }

            _ => {}
        }
    }
}
