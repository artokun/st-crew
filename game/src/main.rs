#![allow(clippy::type_complexity)]

use bevy::{log::LogPlugin, prelude::*};

mod ecs;
mod utils;

use ecs::features::FeaturesPlugin;
use st_commander::CommanderPlugin;

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            CommanderPlugin,
            FeaturesPlugin,
        ))
        .run();
}
