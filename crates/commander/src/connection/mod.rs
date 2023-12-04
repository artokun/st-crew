mod connection_id;

pub use connection_id::*;

use crate::event::{payload::EventPayload, SocketEvent};

#[derive(Debug)]
pub struct SocketConnection {
    pub id: ConnectionId,

    pub(crate) sender: async_channel::Sender<Box<dyn erased_serde::Serialize + Send + Sync>>,
}

impl SocketConnection {
    pub(crate) fn new(
        sender: async_channel::Sender<Box<dyn erased_serde::Serialize + Send + Sync>>,
    ) -> Self {
        Self {
            id: ConnectionId::new(),

            sender,
        }
    }

    pub fn send_event<E>(&self, event: E) -> Result<(), SendError>
    where
        E: SocketEvent,
    {
        // TODO(trevin): add a debug mode check to ensure it's added to the schema

        self.sender
            .try_send(Box::new(EventPayload {
                event: E::NAME,
                payload: event,
            }))
            .map_err(|_| SendError)
    }
}

#[derive(Debug)]
pub struct SendError;
