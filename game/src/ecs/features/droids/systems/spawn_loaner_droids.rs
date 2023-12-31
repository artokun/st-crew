use bevy::{log, prelude::*};

use crate::ecs::features::droids::components::DroidBundle;
use crate::ecs::features::movement::Destination;

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
            DroidBundle {
                name: Name::new(format!("Loaner Droid {}", i + 1)),
                position: Transform::from_translation(Vec3::new(
                    rand::random::<f32>() * 20.0 - 10.0,
                    rand::random::<f32>() * 20.0 - 10.0,
                    0.0,
                )),
                ..Default::default()
            },
            Destination(Vec3::new(*x, *y, 0.0)),
        ));

        log::info!("Spawned loaner droid {} at ({}, {})", i + 1, x, y);
    }
}
