use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerNotFoundContext {
    pub uuid: String,
}
