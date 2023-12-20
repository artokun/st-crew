use std::time::Duration;

use bevy::prelude::*;

mod resources;
mod systems;

pub use resources::*;

pub struct TickPlugin;

const TICKS_PER_SECOND: f32 = 4.0;
const TICK_DURATION: f32 = 1.0 / TICKS_PER_SECOND;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ticks::new())
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(
                TICK_DURATION,
            )))
            .add_systems(FixedUpdate, systems::increment_tick);
    }
}
