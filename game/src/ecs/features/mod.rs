use bevy::app::{App, Plugin};

use self::{
    droids::DroidsPlugin, energy::EnergyPlugin, movement::MovementPlugin, players::PlayersPlugin,
    server::ServerPlugin,
};

pub mod droids;
pub mod energy;
pub mod movement;
pub mod players;
pub mod server;

pub struct FeaturesPlugin;

impl Plugin for FeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugin)
            .add_plugins(EnergyPlugin)
            .add_plugins(PlayersPlugin)
            .add_plugins(DroidsPlugin)
            .add_plugins(MovementPlugin);
    }
}
