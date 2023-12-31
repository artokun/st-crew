pub mod commands;
pub mod event;
pub mod models;
pub mod websocket;

#[cfg(test)]
mod tests {
    use tungstenite::http::Uri;

    use crate::{
        models::{GetPlayerInfoCommand, GetServerInfoCommand},
        websocket::WebsocketSdk,
    };

    #[tokio::test]
    async fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let mut sdk = WebsocketSdk::new(Uri::from_static("ws://127.0.0.1:8081/ws")).await?;

        let response = sdk.execute(GetServerInfoCommand).await?;

        assert_ne!(response.connected_clients, 0);

        let response = sdk
            .execute(GetPlayerInfoCommand {
                uuid: "test".to_string(),
            })
            .await?;

        println!("{:#?}", response.name);

        Ok(())
    }
}
