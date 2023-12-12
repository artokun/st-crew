use bevy::prelude::*;

use self::server::CommanderServer;
use crate::{
    connections::SocketConnections,
    event::{systems::SocketConnectionEventChannel, SocketConnectionEvent},
};

pub mod server;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ReceiveNetworkMessages;

pub struct CommanderPlugin;

impl Plugin for CommanderPlugin {
    fn build(&self, app: &mut App) {
        let (events_tx, events_rx) = async_channel::unbounded();

        app.insert_non_send_resource(CommanderServer::new(events_tx));

        app.add_event::<SocketConnectionEvent>()
            .insert_resource(SocketConnectionEventChannel::new(events_rx))
            .init_resource::<SocketConnections>()
            .add_systems(
                PreUpdate,
                (
                    crate::event::systems::write_events_from_channel,
                    crate::connections::update_connections_map,
                )
                    .chain()
                    .in_set(ReceiveNetworkMessages),
            );
    }
}
