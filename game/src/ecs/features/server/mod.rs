use bevy::{
    app::{App, Plugin, PreUpdate, Startup},
    ecs::schedule::IntoSystemConfigs,
};

mod connected_players;
mod systems;

pub use connected_players::*;

use crate::ecs::plugins::websocket::WsReceiveMessages;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedPlayers::default())
            .add_systems(Startup, systems::startup_socket_listener)
            .add_systems(
                PreUpdate,
                (
                    systems::update_connected_players,
                    systems::handle_message.after(systems::update_connected_players),
                )
                    .after(WsReceiveMessages),
            );
    }
}
