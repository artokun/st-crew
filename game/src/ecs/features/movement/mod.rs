use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

mod components;
mod models;
mod params;
mod socket_events;
mod systems;

pub use components::{Destination, Speed};
pub use models::ImmobileReason;
pub use params::ImmobilityMut;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_positions)
            .add_systems(
                PostUpdate,
                systems::sync_entity_movement.run_if(on_timer(Duration::from_secs(1))),
            );
    }
}
