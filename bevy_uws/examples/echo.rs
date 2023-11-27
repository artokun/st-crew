use bevy::app::{Startup, Update};
use bevy::prelude::{App, Commands, Entity, Query, Res};
use bevy::MinimalPlugins;
use bevy_uws::UWSPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, UWSPlugin))
        .add_systems(Update, receive_message)
        .run();
}

fn receive_message() {}
