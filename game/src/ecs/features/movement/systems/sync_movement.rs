use bevy::prelude::*;
use st_commander::connections::SocketConnections;

use crate::ecs::features::movement::socket_events::{MovementChangedSocketEvent, PositionWithEta};
use crate::ecs::features::{
    common::UniqueId,
    movement::{Destination, Immobile, Speed},
};

pub fn sync_entity_movement(
    query: Query<
        (&UniqueId, &Speed, &Destination, &Transform),
        (With<Destination>, Without<Immobile>),
    >,
    time: Res<Time>,
    connections: Res<SocketConnections>,
) {
    let mut droid_positions = Vec::new();

    for (uuid, speed, destination, transform) in query.iter() {
        let distance = (destination.x - transform.translation.x)
            .hypot(destination.y - transform.translation.y);
        let time_to_arrival = distance / speed.0;
        let server_time = time.elapsed_seconds_wrapped();
        let destination_time = server_time + time_to_arrival;

        let position = PositionWithEta {
            uuid: uuid.0.to_string(),
            origin: (transform.translation.x, transform.translation.y),
            destination: (destination.x, destination.y),
            time_to_arrival,
            server_time,
            destination_time,
        };

        droid_positions.push(position);
    }

    if droid_positions.is_empty() {
        return;
    }

    for (_, connection) in connections.iter() {
        connection
            .send_event(MovementChangedSocketEvent {
                droid_positions: droid_positions.clone(),
            })
            .ok();
    }
}
