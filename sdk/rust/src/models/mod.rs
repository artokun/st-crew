use serde::{Deserialize, Serialize};

use crate::commands::RpcCommand;

mod api_error;

pub use api_error::*;

#[derive(Serialize)]
pub struct GetServerInfoCommand;

impl RpcCommand for GetServerInfoCommand {
    const NAME: &'static str = "get_server_info";

    type Output = GetServerInfoResult;
}

#[derive(Deserialize)]
pub struct GetServerInfoResult {
    pub connected_clients: usize,
}

#[derive(Serialize)]
pub struct GetPlayerInfoCommand {
    pub uuid: String,
}

impl RpcCommand for GetPlayerInfoCommand {
    const NAME: &'static str = "get_player_info";

    type Output = GetPlayerInfoResult;
}

#[derive(Deserialize)]
pub struct GetPlayerInfoResult {
    pub name: String,
}
