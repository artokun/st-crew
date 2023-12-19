use bevy::prelude::*;

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
            .add_systems(PostUpdate, systems::sync_entity_movement);
    }
}
