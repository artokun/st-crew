use bevy::prelude::*;

mod ecs;
pub mod generated;
mod messages;

use ecs::plugins::websocket::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
