use bevy::{log, prelude::*};
use bevy_tokio_tasks::TokioTasksRuntime;
use tokio::{
    net::{TcpListener, ToSocketAddrs},
    task::JoinHandle,
};

use super::{WsConnection, WsEvent};

#[derive(Resource)]
pub struct WsServer {
    events_tx: async_channel::Sender<WsEvent>,

    join_handle: Option<JoinHandle<()>>,
}

impl WsServer {
    pub(crate) fn new(events_tx: async_channel::Sender<WsEvent>) -> Self {
        Self {
            events_tx,

            join_handle: None,
        }
    }

    pub fn start_listening(
        &mut self,
        tokio_runtime: &TokioTasksRuntime,
        bind_to: impl ToSocketAddrs + Send + 'static,
    ) {
        if self.join_handle.is_some() {
            log::error!("websocket server is already listening");
            return;
        }

        log::info!("starting websocket server");

        let events_tx = self.events_tx.clone();

        let task = tokio_runtime.spawn_background_task(move |_| async move {
            let listener = match TcpListener::bind(bind_to).await {
                Ok(listener) => listener,

                Err(err) => {
                    log::error!("error binding websocket listener: {}", err);

                    return;
                }
            };

            WsServer::listen(events_tx, listener).await
        });

        self.join_handle = Some(task);
    }

    // pub fn stop_listening(&mut self) {
    //     log::info!("stopping websocket server");

    //     self.join_handle
    //         .take()
    //         .expect("websocket server is not listening")
    //         .abort();
    // }

    #[cfg(test)]
    pub async fn mock(self) -> crate::ecs::plugins::websocket::mock::MockServer {
        if self.join_handle.is_some() {
            panic!("websocket server is already listening");
        }

        let events_tx = self.events_tx.clone();

        let listener = TcpListener::bind("localhost:0")
            .await
            .expect("failed to bind");

        crate::ecs::plugins::websocket::mock::MockServer::new(
            listener.local_addr().expect("failed to get bound port"),
            self.events_tx,
            tokio::spawn(WsServer::listen(events_tx, listener)),
        )
    }

    async fn listen(events_tx: async_channel::Sender<WsEvent>, listener: TcpListener) {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    tokio::spawn(WsConnection::accept(events_tx.clone(), stream));
                }

                Err(e) => {
                    #[cfg(not(test))]
                    log::error!("error accepting a new connection: {}", e);
                    #[cfg(test)]
                    println!("error accepting a new connection: {}", e);
                }
            }
        }
    }
}

#[derive(Resource)]
pub(super) struct WsEventQueue {
    events_rx: async_channel::Receiver<WsEvent>,
}

impl WsEventQueue {
    pub fn new(events_rx: async_channel::Receiver<WsEvent>) -> Self {
        Self { events_rx }
    }
}

pub(super) fn write_events_from_channel(
    queue: ResMut<WsEventQueue>,
    mut event_writer: EventWriter<WsEvent>,
) {
    while let Ok(event) = queue.events_rx.try_recv() {
        event_writer.send(event);
    }
}
