use bevy::prelude::*;

use crate::ecs::features::{common::UniqueId, movement::Speed};

#[derive(Component)]
pub struct Droid;

#[derive(Bundle)]
pub struct DroidBundle {
    pub droid: Droid,
    pub uuid: UniqueId,
    pub name: Name,
    pub speed: Speed,
    pub position: Transform,
}

impl Default for DroidBundle {
    fn default() -> Self {
        Self {
            droid: Droid,
            uuid: UniqueId::new_random(),
            name: Name::new("Droid".to_string()),
            speed: Speed(1.0),
            position: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
