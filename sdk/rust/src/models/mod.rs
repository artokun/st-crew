use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait RpcCommand {
    const NAME: &'static str;

    type Input: Serialize;
    type Output: DeserializeOwned;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    GetServerInfo,
    GetPlayerInfo { uuid: String },
}
