use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum GetPlayerInfoCommandResult {
    PlayerInfo(PlayerInfo),
    NotFound {
        context: PlayerNotFoundContext,
        error: String,
        message: String,
    },
    Unhandled {
        error: String,
        message: String,
    },
}
