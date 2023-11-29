use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::ecs::plugins::websocket::ReceiveNetworkMessages;

mod messages;
mod systems;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::startup_socket_listener)
            .add_systems(
                Update,
                (systems::send_clients_connected_on_join,).after(ReceiveNetworkMessages),
            );
    }
}
