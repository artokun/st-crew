use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::RpcCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct RpcCall<C> {
    pub id: u64,
    pub command: Cow<'static, str>,
    pub input: C,
}

#[derive(Debug, Serialize)]
pub(crate) struct RpcReply<C>
where
    C: RpcCommand,
{
    pub id: u64,
    pub output: C::Output,
}
