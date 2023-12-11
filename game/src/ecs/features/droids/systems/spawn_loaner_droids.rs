use bevy::{log, prelude::*};

use crate::ecs::features::droids::components::Droid;
use crate::ecs::features::movement::{Destination, Speed};

pub fn spawn_loaner_droids(mut commands: Commands) {
    log::info!("Spawning loaner droids");

    const STARTING_POSITIONS: [(f32, f32); 6] = [
        (0.0, 0.0),
        (0.0, 7.0),
        (5.0, 2.0),
        (10.0, 5.0),
        (2.5, -2.5),
        (-5.0, -5.0),
    ];

    for (i, (x, y)) in STARTING_POSITIONS.iter().enumerate() {
        commands.spawn((
            Droid::new_random(),
            Name::new(format!("Loaner Droid {}", i + 1)),
            Speed(1.0),
            Transform::IDENTITY,
            Destination(Vec3::new(x.to_owned(), y.to_owned(), 0.0)),
        ));

        log::info!("Spawned loaner droid {} at ({}, {})", i + 1, x, y);
    }
}
