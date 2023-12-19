use axum_extra::routing::TypedPath;
use bevy::{ecs::system::Res, log};
use serde::{Deserialize, Serialize};
use st_commander::{
    connections::SocketConnections,
    rpc::{CallError, Rpc, RpcCommand, RpcDispatcher},
};
use st_commander_derive::ApiResponse;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
/// Get information about a player.
pub struct GetPlayerInfoCommand {
    uuid: String,
}

#[axum::async_trait]
impl RpcCommand for GetPlayerInfoCommand {
    const NAME: &'static str = "get_player_info";

    type Output = GetServerInfoResult;
}

#[derive(ApiResponse)]
pub enum GetServerInfoResult {
    /// Success
    #[response(status = OK)]
    Ok(#[from] PlayerInfo),

    #[response(transparent)]
    NotFound(#[from] PlayerNotFoundError),

    #[response(transparent)]
    CallError(#[from] CallError),
}

#[derive(Serialize, ToSchema)]
pub struct PlayerInfo {
    pub name: String,
}

#[derive(Debug, thiserror::Error, ToSchema, ApiResponse)]
/// The given player was not found.
#[response(status = NOT_FOUND)]
#[error("player not found")]
pub struct PlayerNotFoundError {
    pub uuid: String,
}

#[derive(TypedPath, Deserialize, ToSchema)]
#[typed_path("/players/:uuid")]
/// Get player info
///
/// Get information about a player.
pub struct GetPlayerInfoRoute {
    uuid: String,
}

pub async fn route_get_player_info(
    GetPlayerInfoRoute { uuid }: GetPlayerInfoRoute,
    rpc: RpcDispatcher<GetPlayerInfoCommand>,
) -> GetServerInfoResult {
    // TODO: check jwt token

    rpc.call(GetPlayerInfoCommand { uuid }).await.into()
}

pub fn on_player_info_command(rpc: Rpc<GetPlayerInfoCommand>, connections: Res<SocketConnections>) {
    for cmd in rpc {
        log::info!("handling get player info command: {:?}", cmd.uuid);

        cmd.reply(PlayerInfo {
            name: "test".to_string(),
        });
    }
}
