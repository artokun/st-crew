use serde::{Deserialize, Serialize};

use crate::command::RpcCommand;

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
pub struct GetPlayerInfoCommand {}

impl RpcCommand for GetPlayerInfoCommand {
    const NAME: &'static str = "get_player_info";

    type Output = GetPlayerInfoResult;
}

#[derive(Deserialize)]
pub struct GetPlayerInfoResult {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    GetServerInfo,
    GetPlayerInfo { uuid: String },
}
