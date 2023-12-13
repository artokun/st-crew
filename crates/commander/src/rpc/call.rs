use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::RpcCommand;

#[derive(Debug, Deserialize, ToSchema)]
pub(crate) struct RpcCallCommand {
    pub id: u64,
    pub command: Cow<'static, str>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub(crate) struct RpcCall<C>
where
    C: RpcCommand,
{
    pub id: u64,
    pub command: Cow<'static, str>,
    pub input: C::Input,
}

#[derive(Debug, Serialize)]
pub(crate) struct RpcReply<C>
where
    C: RpcCommand,
{
    pub id: u64,
    pub output: C::Output,
}
