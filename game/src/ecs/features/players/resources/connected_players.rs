use bevy::{
    ecs::{entity::Entity, system::Resource},
    prelude::Deref,
    utils::HashMap,
};

use crate::ecs::plugins::websocket::ConnectionId;

#[derive(Resource, Default, Deref)]
pub struct ConnectedPlayers {
    map: HashMap<ConnectionId, Entity>,
}

impl ConnectedPlayers {
    pub fn on_player_connected(&mut self, connection_id: ConnectionId, player: Entity) {
        self.map.insert(connection_id, player);
    }

    pub fn on_player_disconnected(&mut self, connection_id: &ConnectionId) -> Option<Entity> {
        self.map.remove(connection_id)
    }
}
