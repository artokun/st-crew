use bevy::prelude::*;
use st_commander::{connection::ConnectionId, connections::SocketConnections};

use crate::ecs::features::energy::{components::Energy, socket_events::EnergyChangedSocketEvent};

pub fn sync_player_energy(
    mut query: Query<(&ConnectionId, &Energy), Changed<Energy>>,
    connections: Res<SocketConnections>,
) {
    for (connection_id, new_energy) in query.iter_mut() {
        let connection = connections.get(connection_id).expect("no connection found");

        connection
            .send_event(EnergyChangedSocketEvent {
                current: new_energy.current,
                capacity: new_energy.capacity,
            })
            .ok();
    }
}
