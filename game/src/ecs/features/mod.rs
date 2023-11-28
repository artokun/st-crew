use bevy::app::{App, Plugin};

use self::server::ServerPlugin;

pub mod server;

pub struct FeaturesPlugin;

impl Plugin for FeaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ServerPlugin);
    }
}
