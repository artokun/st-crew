use bevy::{log::LogPlugin, prelude::*};

mod components;
pub mod generated;
mod messages;
mod plugins;

use plugins::sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default(), WebSocketPlugin))
        .run();
}
