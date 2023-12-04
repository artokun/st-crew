use std::sync::Arc;

use bevy::utils::HashMap;

use crate::{event::SocketConnectionEvent, rpc::RpcDispatch};

use super::CommanderSchema;

pub struct CommanderState {
    inner: Arc<InnerCommanderState>,
}

impl CommanderState {
    pub(crate) fn new(
        events_tx: async_channel::Sender<SocketConnectionEvent>,
        commands: HashMap<&'static str, Box<dyn RpcDispatch>>,
        schema: CommanderSchema,
    ) -> Self {
        Self {
            inner: Arc::new(InnerCommanderState {
                events_tx,

                commands,

                schema,
            }),
        }
    }

    pub(crate) fn into_inner(self) -> Arc<InnerCommanderState> {
        self.inner
    }
}

impl Clone for CommanderState {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub(crate) struct InnerCommanderState {
    pub events_tx: async_channel::Sender<SocketConnectionEvent>,

    pub commands: HashMap<&'static str, Box<dyn RpcDispatch>>,

    pub schema: CommanderSchema,
}
