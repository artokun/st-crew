use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
    pub connected_clients: usize,
}
