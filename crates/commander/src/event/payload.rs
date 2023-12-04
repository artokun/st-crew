use serde::Serialize;

use super::SocketEvent;

#[derive(Serialize)]
pub struct EventPayload<E>
where
    E: SocketEvent,
{
    pub event: &'static str,
    pub payload: E,
}
