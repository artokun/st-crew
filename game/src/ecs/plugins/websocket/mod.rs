use bevy::{ecs::system::SystemParam, log, prelude::*};
use bevy_tokio_tasks::TokioTasksRuntime;
use tokio::net::{TcpListener, ToSocketAddrs};

mod connection;
mod connection_id;
mod connections;
mod event;
mod listener;
mod message;

pub use connection::*;
pub use connection_id::*;
pub use connections::*;
pub use event::*;
pub use message::*;

use self::listener::{write_events_from_channel, WsEventQueue, WsListener};

// TODO: lets add the connection state resource here as well as define the message event types,
// lets also create a fixed time step scheduler to handle the energy generation

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        let (events_tx, events_rx) = async_channel::unbounded();

        app.insert_resource(WsListener {
            events_tx,

            join_handle: None,
        })
        .insert_resource(WsEventQueue { events_rx })
        .insert_resource(WsConnections::default())
        .add_event::<WsEvent>()
        .add_systems(
            PreUpdate,
            (write_events_from_channel, update_connections_map).chain(),
        );
    }
}

#[derive(SystemParam)]
pub struct WsServer<'w> {
    tokio_runtime: Res<'w, TokioTasksRuntime>,
    ws_server: ResMut<'w, WsListener>,
}

impl WsServer<'_> {
    pub fn start_listening(&mut self, bind_to: impl ToSocketAddrs + Send + 'static) {
        if self.ws_server.join_handle.is_some() {
            log::error!("websocket server is already listening");
            return;
        }

        log::info!("starting websocket server");

        let events_tx = self.ws_server.events_tx.clone();

        let task = self
            .tokio_runtime
            .spawn_background_task(move |_| async move {
                let listener = match TcpListener::bind(bind_to).await {
                    Ok(listener) => listener,

                    Err(err) => {
                        log::error!("error binding websocket listener: {}", err);

                        return;
                    }
                };

                loop {
                    match listener.accept().await {
                        Ok((stream, _)) => {
                            tokio::spawn(WsConnection::accept(events_tx.clone(), stream));
                        }

                        Err(e) => {
                            log::error!("error accepting a new connection: {}", e);
                        }
                    }
                }
            });

        self.ws_server.join_handle = Some(task);
    }
}
