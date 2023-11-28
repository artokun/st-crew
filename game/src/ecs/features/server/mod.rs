use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

mod connected_players;
mod systems;

pub use connected_players::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedPlayers::default())
            .add_systems(Startup, systems::startup_socket_listener)
            .add_systems(
                Update,
                (
                    systems::update_connected_players,
                    systems::handle_message.after(systems::update_connected_players),
                ),
            );
    }
}
