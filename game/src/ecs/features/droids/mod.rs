use bevy::prelude::*;

mod components;
mod systems;

pub use components::Droid;

pub struct DroidsPlugin;

impl Plugin for DroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_loaner_droids);
    }
}
