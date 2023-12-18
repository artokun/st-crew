use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
}
