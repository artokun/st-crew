use std::env;

use bevy::app::{App, Plugin};
use st_commander::CommanderServerExt;
use utoipa::openapi::security::{
    AuthorizationCode, Flow, HttpAuthScheme, HttpBuilder, OAuth2, Password, Scopes, SecurityScheme,
};

use self::{
    droids::DroidsPlugin, energy::EnergyPlugin, movement::MovementPlugin, players::PlayersPlugin,
    server::ServerPlugin,
};

pub mod common;
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

            schema.components = schema
                .components
                .security_scheme(
                    "jwt",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .security_scheme(
                    "oath2",
                    SecurityScheme::OAuth2(OAuth2::new([
                        Flow::Password(Password::with_refresh_url(
                            "https://localhost/oauth/token",
                            Scopes::from_iter([
                                ("edit:items", "edit my items"),
                                ("read:items", "read my items"),
                            ]),
                            "https://localhost/refresh/token",
                        )),
                        Flow::AuthorizationCode(AuthorizationCode::new(
                            "https://localhost/authorization/token",
                            "https://localhost/token/url",
                            Scopes::from_iter([
                                ("edit:items", "edit my items"),
                                ("read:items", "read my items"),
                            ]),
                        )),
                    ])),
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
