use bevy::{log::LogPlugin, prelude::*};
use bevy_tokio_tasks::TokioTasksPlugin;

mod ecs;
pub mod generated;

use ecs::{features::FeaturesPlugin, plugins::websocket::WebSocketPlugin};

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            LogPlugin::default(),
            TokioTasksPlugin::default(),
            WebSocketPlugin,
            FeaturesPlugin,
        ))
        .run();
}
