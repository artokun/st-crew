use bevy::prelude::*;
use bevy_ws_server::WsPlugin;

use crate::ecs::systems::sockets::{
    assign_client_to_socket, receive_message, startup_socket_listener,
};

// TODO: lets add the connection state resource here as well as define the message event types,
// lets also create a fixed time step scheduler to handle the energy generation

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WsPlugin)
            .add_systems(Startup, startup_socket_listener)
            .add_systems(Update, (assign_client_to_socket, receive_message));
    }
}
