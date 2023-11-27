use bevy::prelude::*;

mod generated;
mod messages;
mod sockets;

use sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
