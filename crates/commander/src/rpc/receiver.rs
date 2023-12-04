use bevy::{
    ecs::system::{ResMut, Resource, SystemParam},
    prelude::{Deref, DerefMut},
};
use tokio::sync::mpsc;

use crate::rpc::{RpcCommand, RpcRequest};

#[derive(SystemParam)]
pub struct Rpc<'w, C: RpcCommand> {
    rpc_channel: ResMut<'w, RpcChannel<C>>,
}

impl<C> Iterator for Rpc<'_, C>
where
    C: RpcCommand,
{
    type Item = RpcRequest<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rpc_channel.try_recv().ok()
    }
}

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct RpcChannel<C: RpcCommand> {
    rx: mpsc::UnboundedReceiver<RpcRequest<C>>,
}

impl<C> RpcChannel<C>
where
    C: RpcCommand,
{
    pub(crate) fn new(rx: mpsc::UnboundedReceiver<RpcRequest<C>>) -> Self {
        Self { rx }
    }
}
