use axum::http::StatusCode;
use axum_extra::routing::TypedPath;
use bevy::{ecs::system::Res, log};
use serde::Deserialize;
use st_commander::{
    connections::SocketConnections,
    response::{ApiResponse, ApiResult},
    rpc::{NoInput, Rpc, RpcCommand, RpcDispatcher},
};
use utoipa::ToSchema;

use crate::ecs::features::server::models::ServerInfo;

#[derive(Deserialize, ToSchema)]
pub struct GetServerInfoCommand;

impl RpcCommand for GetServerInfoCommand {
    const NAME: &'static str = "get_server_info";

    type Input = NoInput;
    type Output = ServerInfo;
}

#[derive(TypedPath)]
#[typed_path("/server-info")]
pub struct GetServerInfoRoute;

pub async fn route_get_server_info(
    _: GetServerInfoRoute,
    rpc: RpcDispatcher<GetServerInfoCommand>,
) -> ApiResult<ServerInfo> {
    log::info!("dispatching get server info command");

    let output = rpc.call(()).await?;

    log::info!("received response to get server info command");

    Ok(ApiResponse::new(StatusCode::OK).with_body(output))
}

pub fn on_server_info_command(rpc: Rpc<GetServerInfoCommand>, connections: Res<SocketConnections>) {
    for cmd in rpc {
        log::info!("handling get server info command");

        cmd.reply(ServerInfo {
            connected_clients: connections.iter().count(),
        });
    }
}
