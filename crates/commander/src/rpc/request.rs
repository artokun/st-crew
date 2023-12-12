use tokio::sync::oneshot;

use super::RpcCommand;

pub struct RpcRequest<C>
where
    C: RpcCommand,
{
    pub input: C::Input,

    pub(super) reply_tx: oneshot::Sender<C::Output>,
}

impl<C> RpcRequest<C>
where
    C: RpcCommand,
{
    pub fn reply(self, response: impl Into<C::Output>) {
        // TODO: Do we want to handle this error?
        self.reply_tx.send(response.into()).ok();
    }
}
