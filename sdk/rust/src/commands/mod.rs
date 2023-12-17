use serde::{de::DeserializeOwned, Serialize};

pub trait RpcCommand: Serialize {
    const NAME: &'static str;

    type Output: DeserializeOwned;
}
