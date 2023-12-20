use bevy::prelude::*;

use crate::ecs::features::tick::Ticks;

pub fn increment_tick(mut ticks: ResMut<Ticks>) {
    ticks.tick();
}
