use bevy::prelude::*;

use crate::ecs::features::{
    common::{UniqueId, UniqueIdType},
    movement::Speed,
};

pub struct Droid;

impl UniqueIdType for Droid {}

#[derive(Bundle)]
pub struct DroidBundle {
    pub id: UniqueId<Droid>,
    pub name: Name,
    pub speed: Speed,
    pub position: Transform,
}

impl Default for DroidBundle {
    fn default() -> Self {
        Self {
            id: UniqueId::new_random(),
            name: Name::new("Droid"),
            speed: Speed::per_second(1.0),
            position: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
