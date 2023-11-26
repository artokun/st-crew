mod generated;
mod messages;
mod sockets;

use bevy::prelude::*;
use sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
