use bevy::{log::LogPlugin, prelude::*};

mod api;
mod ecs;

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
