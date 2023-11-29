use bevy::prelude::*;

pub mod generated;
mod messages;
mod player;
mod sockets;

use sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
