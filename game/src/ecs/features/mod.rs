use bevy::app::{App, Plugin};

use self::{energy::EnergyPlugin, players::PlayersPlugin, server::ServerPlugin};

pub mod energy;
pub mod players;
pub mod server;

pub struct FeaturesPlugin;

impl Plugin for FeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugin)
            .add_plugins(EnergyPlugin)
            .add_plugins(PlayersPlugin);
    }
}
