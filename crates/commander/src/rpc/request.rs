use bevy::prelude::Deref;
use tokio::sync::oneshot;

use super::RpcCommand;

#[derive(Deref)]
pub struct RpcRequest<C>
where
    C: RpcCommand,
{
    #[deref]
    pub(super) input: C::Input,

    pub(super) reply_tx: oneshot::Sender<C::Output>,
}

impl<C> RpcRequest<C>
where
    C: RpcCommand,
{
    pub fn reply(self, response: C::Output) {
        // TODO: Do we want to handle this error?
        self.reply_tx.send(response).ok();
    }
}
