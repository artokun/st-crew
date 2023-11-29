use bevy::prelude::*;

use crate::ecs::{
    features::energy::components::Energy,
    plugins::websocket::{ConnectionId, WsConnections, WsMessage},
};

pub fn sync_player_energy(
    mut query: Query<(&ConnectionId, &Energy), Changed<Energy>>,
    connections: Res<WsConnections>,
) {
    for (connection_id, new_energy) in query.iter_mut() {
        let connection = connections.get(connection_id).expect("no connection found");

        connection
            .send_raw(WsMessage::Text(format!(
                "Your energy has changed! You now have {}/{}.",
                new_energy.current, new_energy.capacity
            )))
            .ok();
    }
}
