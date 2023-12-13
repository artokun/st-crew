use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::RpcCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct RpcCall<I> {
    pub id: u64,
    pub command: Cow<'static, str>,
    pub input: I,
}

#[derive(Debug, Serialize)]
pub(crate) struct RpcReply<C>
where
    C: RpcCommand,
{
    pub id: u64,
    pub output: C::Output,
}
