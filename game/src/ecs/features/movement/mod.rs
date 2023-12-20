use std::time::Duration;

use bevy::{ecs::schedule::IntoSystemConfigs, prelude::*, time::common_conditions::on_timer};

mod components;
mod models;
mod params;
mod socket_events;
mod systems;

pub use components::{Destination, Immobile, Speed};
pub use models::ImmobileReason;
pub use params::ImmobilityMut;

#[derive(Resource, Deref)]
pub struct Ticks(u64);

fn increment_tick(mut ticks: ResMut<Ticks>) {
    ticks.0 += 1;
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ticks(0))
            .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(250)))
            .add_systems(
                FixedUpdate,
                (
                    increment_tick,
                    systems::update_positions.after(increment_tick),
                ),
            )
            .add_systems(
                PostUpdate,
                systems::sync_entity_movement.run_if(on_timer(Duration::from_secs(1))),
            );
    }
}
