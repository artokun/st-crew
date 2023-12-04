use bevy::prelude::*;

mod components;
mod resources;
mod systems;

pub use components::Player;
pub use resources::ConnectedPlayers;
use st_commander::ReceiveNetworkMessages;
pub use systems::SpawnPlayer;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConnectedPlayers::default())
            .configure_sets(PreUpdate, SpawnPlayer.after(ReceiveNetworkMessages))
            .add_systems(
                PreUpdate,
                systems::update_connected_players.in_set(SpawnPlayer),
            );
    }
}
