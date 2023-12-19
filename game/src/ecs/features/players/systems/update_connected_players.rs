use bevy::{
    core::Name,
    ecs::{
        bundle::Bundle,
        event::EventReader,
        system::{Commands, ResMut},
    },
    log,
};
use st_commander::{connection::ConnectionId, event::SocketConnectionEvent};

use crate::ecs::features::{
    common::UniqueId,
    players::{ConnectedPlayers, Player},
};

#[derive(Bundle)]
struct PlayerBundle {
    connection_id: ConnectionId,
    player: Player,
    uuid: UniqueId,
    name: Name,
}

pub fn update_connected_players(
    mut commands: Commands,
    mut event_reader: EventReader<SocketConnectionEvent>,
    mut connected_players: ResMut<ConnectedPlayers>,
) {
    for event in event_reader.read() {
        match event {
            SocketConnectionEvent::Connected { connection } => {
                let uuid = UniqueId::new_random();

                let name = Name::new(format!("player-{}", &uuid.to_string()[..8]));

                // If the player entity exists before they connect, find the entity and attach them here
                let entity = commands.spawn(PlayerBundle {
                    connection_id: connection.id,
                    player: Player,
                    uuid,
                    name,
                });

                log::info!("spawning player with entity id: {:?}", entity.id());

                connected_players.on_player_connected(connection.id, entity.id());
            }

            SocketConnectionEvent::Disconnected { connection_id } => {
                match connected_players.on_player_disconnected(connection_id) {
                    Some(entity) => {
                        commands.entity(entity).despawn();
                    }

                    None => {
                        log::warn!("no entity found for connection id: {}", connection_id);
                    }
                }
            }
        }
    }
}
