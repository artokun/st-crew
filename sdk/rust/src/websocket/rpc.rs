use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RpcCall<C> {
    pub id: u64,
    pub command: &'static str,
    pub input: C,
}

#[derive(Debug, Deserialize)]
pub struct RpcReply<O> {
    pub id: u64,
    pub output: O,
}
