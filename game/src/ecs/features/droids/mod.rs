use bevy::prelude::*;

pub mod components;
mod systems;

pub use systems::spawn_loaner_droids;

pub struct DroidsPlugin;

impl Plugin for DroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_loaner_droids);
    }
}
