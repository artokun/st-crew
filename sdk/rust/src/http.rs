use crate::command::RpcCommand;

pub struct HttpSdk {
    client: reqwest::Client,
}

impl HttpSdk {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn execute<C>(&self) -> C::Output
    where
        C: RpcCommand,
    {
    }
}
