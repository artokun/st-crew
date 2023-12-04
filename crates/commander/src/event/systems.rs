use bevy::ecs::{
    event::EventWriter,
    system::{ResMut, Resource},
};

use super::SocketConnectionEvent;

#[derive(Resource)]
pub struct SocketConnectionEventChannel {
    events_rx: async_channel::Receiver<SocketConnectionEvent>,
}

impl SocketConnectionEventChannel {
    pub fn new(events_rx: async_channel::Receiver<SocketConnectionEvent>) -> Self {
        Self { events_rx }
    }
}

pub fn write_events_from_channel(
    channel: ResMut<SocketConnectionEventChannel>,
    mut event_writer: EventWriter<SocketConnectionEvent>,
) {
    while let Ok(event) = channel.events_rx.try_recv() {
        event_writer.send(event);
    }
}
