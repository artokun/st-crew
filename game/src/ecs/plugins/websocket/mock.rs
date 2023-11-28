use std::net::SocketAddr;

use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::http::Uri;

use super::WsEvent;

pub struct MockServer {
    addr: SocketAddr,

    events_tx: async_channel::Sender<WsEvent>,

    join_handle: Option<JoinHandle<()>>,
}

impl MockServer {
    pub fn new(
        addr: SocketAddr,
        events_tx: async_channel::Sender<WsEvent>,
        join_handle: JoinHandle<()>,
    ) -> Self {
        Self {
            addr,

            events_tx,

            join_handle: Some(join_handle),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn uri(&self) -> Uri {
        Uri::builder()
            .scheme("ws")
            .authority(self.addr.to_string().as_str())
            .path_and_query("/")
            .build()
            .unwrap()
    }

    pub async fn send(&self, event: WsEvent) -> Result<(), async_channel::SendError<WsEvent>> {
        self.events_tx.send(event).await
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.join_handle.take().unwrap().abort();
    }
}
