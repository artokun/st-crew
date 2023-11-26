use bevy::prelude::*;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
mod messages;
mod sockets;

use sockets::WebSocketPlugin;

fn main() {
    App::new()
        .add_plugins((MinimalPlugins, WebSocketPlugin))
        .run();
}
