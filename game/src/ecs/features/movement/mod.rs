use bevy::prelude::*;

pub mod components;
mod systems;

pub use systems::update_positions;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_positions);
    }
}
