use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SocketEventMessage<E> {
    pub event: String,
    pub payload: E,
}
