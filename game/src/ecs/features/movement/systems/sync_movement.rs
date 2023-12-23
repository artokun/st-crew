use bevy::prelude::*;
use st_commander::connections::SocketConnections;

use crate::ecs::features::tick::TickTimer;
use crate::ecs::features::{
    common::UniqueId,
    movement::{Destination, Speed},
};
use crate::ecs::features::{
    droids::Droid,
    movement::{
        components::Immobile,
        socket_events::{MovementChangedSocketEvent, PositionWithEta},
        Distance,
    },
};

pub fn sync_entity_movement(
    query: Query<
        (&UniqueId<Droid>, &Speed, &Destination, &Transform),
        (With<Destination>, Without<Immobile>),
    >,
    tick: Res<TickTimer>,
    connections: Res<SocketConnections>,
) {
    let mut droid_positions = Vec::new();

    for (id, speed, destination, transform) in query.iter() {
        let distance = Distance::between_positions(transform.translation, destination.0);
        let remaining_ticks = distance / *speed;
        let current_tick = tick.current_tick();
        let arrival_tick = current_tick + remaining_ticks;

        let position = PositionWithEta {
            uuid: *id.as_ref(),
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
