use async_uws::app::App as UWSApp;
use async_uws::app::AppStruct;
use async_uws::http_response::HttpResponse;
use async_uws::uwebsockets_rs::CompressOptions;
use async_uws::uwebsockets_rs::UsSocketContextOptions;
use async_uws::websocket::Websocket;
use async_uws::ws_behavior::WsRouteSettings;
use async_uws::ws_message::WsMessage;
use bevy::prelude::*;
use std::sync::Arc;
use tokio::runtime::Runtime;

use uwebsockets_rs::listen_socket::ListenSocket;

struct TokioRuntime(Arc<Runtime>);

impl Resource for TokioRuntime {}

pub struct UWSPlugin;

impl Plugin for UWSPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the Tokio runtime
        let tokio_runtime = Arc::new(Runtime::new().unwrap());

        // Wrap the runtime in your custom struct
        let runtime_resource = TokioRuntime(tokio_runtime);

        // Add the wrapper struct as a Bevy resource
        app.insert_resource(runtime_resource);

        // Startup system to initialize the websocket listener
        app.add_systems(Startup, setup_listener);
        app.add_systems(Update, accept_connections);
    }
}

fn setup_listener(tokio_runtime: Res<TokioRuntime>) {
    async fn handler_ws(mut ws: Websocket<false>) {
        println!("New connection");
        while let Some(msg) = ws.stream.recv().await {
            if let WsMessage::Close(_, _) = msg {
                break;
            }
            let res = ws.send(msg).await;
            if res.is_err() {
                println!("1");
            }
        }
        println!("Done with that websocket!");
    }
    tokio_runtime.0.spawn(async move {
        let compressor: u32 = CompressOptions::SharedCompressor.into();
        let decompressor: u32 = CompressOptions::SharedDecompressor.into();
        let route_settings = WsRouteSettings {
            compression: Some(compressor | decompressor),
            max_payload_length: Some(1024),
            idle_timeout: Some(800),
            max_backpressure: Some(10),
            close_on_backpressure_limit: Some(false),
            reset_idle_timeout_on_send: Some(true),
            send_pings_automatically: Some(true),
            max_lifetime: Some(111),
        };
        let default_options: UsSocketContextOptions = UsSocketContextOptions {
            key_file_name: None,
            cert_file_name: None,
            passphrase: None,
            dh_params_file_name: None,
            ca_file_name: None,
            ssl_ciphers: None,
            ssl_prefer_low_memory_usage: None,
        };
        let mut app = UWSApp::new(default_options, None);
        app.ws(
            "/",
            route_settings.clone(),
            handler_ws,
            HttpResponse::default_upgrade,
        )
        .listen(9001, None::<fn(ListenSocket)>)
        .run();
    });
}

fn accept_connections() {}
