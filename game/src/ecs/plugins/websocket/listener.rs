use bevy::prelude::*;
use tokio::task::JoinHandle;

use super::WsEvent;

#[derive(Resource)]
pub struct WsListener {
    pub events_tx: async_channel::Sender<WsEvent>,

    pub join_handle: Option<JoinHandle<()>>,
}

#[derive(Resource)]
pub struct WsEventQueue {
    pub events_rx: async_channel::Receiver<WsEvent>,
}

pub(super) fn write_events_from_channel(
    queue: ResMut<WsEventQueue>,
    mut event_writer: EventWriter<WsEvent>,
) {
    while let Ok(event) = queue.events_rx.try_recv() {
        event_writer.send(event);
    }
}
