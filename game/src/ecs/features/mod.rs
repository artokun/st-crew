use std::env;

use bevy::app::{App, Plugin};
use st_commander::CommanderServerExt;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

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
        app.with_schema(|mut schema| {
            schema.info = schema
                .info
                .title(env!("CARGO_PKG_NAME"))
                .version(env!("CARGO_PKG_VERSION"))
                .description(Some(env!("CARGO_PKG_DESCRIPTION")));
            // .terms_of_service(Some("https://st"));

            schema.components = schema.components.security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("apikey"))),
            );

            schema
        })
        .add_plugins(ServerPlugin)
        .add_plugins(EnergyPlugin)
        .add_plugins(PlayersPlugin)
        .add_plugins(DroidsPlugin)
        .add_plugins(MovementPlugin);
    }
}
