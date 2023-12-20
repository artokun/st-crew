use bevy::{log, prelude::*};

use crate::ecs::features::movement::components::{Destination, Immobile, Speed};

pub fn update_positions(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Name, &Speed, &Destination, &mut Transform),
        (With<Destination>, Without<Immobile>),
    >,
    // time: Res<Time>,
) {
    for (entity, name, speed, destination, mut transform) in query.iter_mut() {
        // each tick, move towards destination
        let direction = Vec3::new(
            destination.x - transform.translation.x,
            destination.y - transform.translation.y,
            0.0,
        )
        .normalize();

        transform.translation += direction * speed.0;

        // log::info!(
        //     "Moving {} from ({}, {}) to ({}, {})",
        //     name,
        //     transform.translation.x,
        //     transform.translation.y,
        //     destination.x,
        //     destination.y
        // );

        if (transform.translation.x - destination.x).abs() <= speed.0
            && (transform.translation.y - destination.y).abs() <= speed.0
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
