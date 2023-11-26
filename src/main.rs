mod sockets;
mod messages;
mod generated;

use bevy::prelude::*;
use sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
