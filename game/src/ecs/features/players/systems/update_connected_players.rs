use bevy::{
    core::Name,
    ecs::{
        bundle::Bundle,
        event::EventReader,
        system::{Commands, ResMut},
    },
    log,
};

use crate::ecs::{
    features::players::{ConnectedPlayers, Player},
    plugins::websocket::{ConnectionId, WsEvent},
};

#[derive(Bundle)]
struct PlayerBundle {
    connection_id: ConnectionId,
    player: Player,
    name: Name,
}

pub fn update_connected_players(
    mut commands: Commands,
    mut event_reader: EventReader<WsEvent>,
    mut connected_players: ResMut<ConnectedPlayers>,
) {
    for event in event_reader.read() {
        match event {
            WsEvent::Connected { connection } => {
                let player = Player::new_random();

                let name = Name::new(format!("player-{}", &player.to_string()[..8]));

                // If the player entity exists before they connect, find the entity and attach them here
                let entity = commands.spawn(PlayerBundle {
                    connection_id: connection.id,
                    player,
                    name,
                });

                log::info!("spawning player with entity id: {:?}", entity.id());

                connected_players.on_player_connected(connection.id, entity.id());
            }

            WsEvent::Disconnected { connection_id } => {
                match connected_players.on_player_disconnected(connection_id) {
                    Some(entity) => {
                        commands.entity(entity).despawn();
                    }

                    None => {
                        log::warn!("no entity found for connection id: {}", connection_id);
                    }
                }
            }

            _ => {}
        }
    }
}
