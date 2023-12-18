use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum GetServerInfoCommandResult {
    ServerInfo(ServerInfo),
    Unhandled {
        error: String,
        message: String,
    },
}
