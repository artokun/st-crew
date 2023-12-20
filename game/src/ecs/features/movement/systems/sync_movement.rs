use bevy::prelude::*;
use st_commander::connections::SocketConnections;

use crate::ecs::features::movement::socket_events::{MovementChangedSocketEvent, PositionWithEta};
use crate::ecs::features::tick::Ticks;
use crate::ecs::features::{
    common::UniqueId,
    movement::{Destination, Immobile, Speed},
};

pub fn sync_entity_movement(
    query: Query<
        (&UniqueId, &Speed, &Destination, &Transform),
        (With<Destination>, Without<Immobile>),
    >,
    tick: Res<Ticks>,
    connections: Res<SocketConnections>,
) {
    let mut droid_positions = Vec::new();

    for (uuid, speed, destination, transform) in query.iter() {
        let distance = (destination.x - transform.translation.x)
            .hypot(destination.y - transform.translation.y);
        let remaining_ticks = (distance / speed.0).ceil() as u32;
        let current_tick = tick.0;
        let arrival_tick = current_tick + remaining_ticks as u64;

        let position = PositionWithEta {
            uuid: uuid.0.to_string(),
            origin: (transform.translation.x, transform.translation.y),
            destination: (destination.x, destination.y),
            current_tick,
            arrival_tick,
        };

        droid_positions.push(position);
    }

    if droid_positions.is_empty() {
        return;
    }

    // log::info!("{:#?}", droid_positions);

    for (_, connection) in connections.iter() {
        connection
            .send_event(MovementChangedSocketEvent {
                droid_positions: droid_positions.clone(),
            })
            .ok();
    }
}
