use bevy::{log, prelude::*};

use crate::ecs::features::movement::components::{Destination, Speed};

pub fn update_positions(
    mut commands: Commands,
    mut query: Query<(Entity, &Name, &Speed, &Destination, &mut Transform), With<Destination>>,
    time: Res<Time>,
) {
    for (entity, name, speed, destination, mut transform) in query.iter_mut() {
        transform.translation.x +=
            (destination.x - transform.translation.x).signum() * speed.0 * time.delta_seconds();
        transform.translation.y +=
            (destination.y - transform.translation.y).signum() * speed.0 * time.delta_seconds();

        // log::info!(
        //     "Moving {} from ({}, {}) to ({}, {})",
        //     name,
        //     transform.translation.x,
        //     transform.translation.y,
        //     destination.x,
        //     destination.y
        // );

        if (transform.translation.x - destination.x).abs() < 0.001
            && (transform.translation.y - destination.y).abs() < 0.001
        {
            transform.translation = destination.0;
            commands.entity(entity).remove::<Destination>();

            log::info!(
                "{} has arrived at ({}, {})",
                name,
                destination.x,
                destination.y
            );
        }
    }
}
