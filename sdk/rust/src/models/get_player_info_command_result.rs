use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum GetPlayerInfoCommandResult {
    PlayerInfo(PlayerInfo),
    Unhandled {
        error: String,
        message: String,
    },
}
