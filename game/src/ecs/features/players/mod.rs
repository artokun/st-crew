use bevy::prelude::*;

mod commands;
mod components;
mod resources;
mod systems;

pub use components::Player;
pub use resources::ConnectedPlayers;
use st_commander::{CommanderServerExt, ReceiveNetworkMessages};
pub use systems::SpawnPlayer;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.register_command::<commands::get_player_info::GetPlayerInfoCommand>()
            .get(commands::get_player_info::route_get_player_info)
            .add_systems(Update, commands::get_player_info::on_player_info_command);

        app.insert_resource(ConnectedPlayers::default())
            .configure_sets(PreUpdate, SpawnPlayer.after(ReceiveNetworkMessages))
            .add_systems(
                PreUpdate,
                systems::update_connected_players.in_set(SpawnPlayer),
            );
    }
}
