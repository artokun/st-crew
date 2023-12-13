use serde::de::DeserializeOwned;

pub trait SocketEvent: DeserializeOwned {
    const NAME: &'static str;
}
