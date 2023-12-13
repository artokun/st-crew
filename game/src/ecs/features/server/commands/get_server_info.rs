use axum_extra::routing::TypedPath;
use bevy::{ecs::system::Res, log};
use serde::Deserialize;
use st_commander::{
    connections::SocketConnections,
    rpc::{CallError, Rpc, RpcCommand, RpcDispatcher},
};
use st_commander_derive::ApiResponse;
use utoipa::ToSchema;

use crate::ecs::features::server::models::ServerInfo;

#[derive(Deserialize, ToSchema)]
/// Get information about the current state of the server.
pub struct GetServerInfoCommand;

#[axum::async_trait]
impl RpcCommand for GetServerInfoCommand {
    const NAME: &'static str = "get_server_info";

    type Input = ();
    type Output = GetServerInfoResult;
}

#[derive(ApiResponse)]
pub enum GetServerInfoResult {
    /// Success
    #[response(status = OK)]
    Ok(#[from] ServerInfo),

    #[response(transparent)]
    CallError(#[from] CallError),
}

#[derive(TypedPath, Deserialize, ToSchema)]
#[typed_path("/server-info")]
/// Get server info
///
/// Get information about the current state of the server.
pub struct GetServerInfoRoute {}

pub async fn route_get_server_info(
    _: GetServerInfoRoute,
    rpc: RpcDispatcher<GetServerInfoCommand>,
) -> GetServerInfoResult {
    // TODO: check jwt token

    rpc.call(()).await.into()
}

pub fn on_server_info_command(rpc: Rpc<GetServerInfoCommand>, connections: Res<SocketConnections>) {
    for cmd in rpc {
        log::info!("handling get server info command");

        cmd.reply(ServerInfo {
            connected_clients: connections.iter().count(),
        });
    }
}
