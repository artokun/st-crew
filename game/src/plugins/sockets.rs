use async_tungstenite::tungstenite::protocol::Message as WSMessage;
use bevy::{log, prelude::*};
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};

use crate::components::player::{Energy, Name, PlayerBundle};
use crate::generated::message::{Message, MessageType};
use crate::messages::server_stat_event;

const GENERATE_ENERGY_INTERVAL: f32 = 1.0;

#[derive(Resource)]
struct GenerateEnergyTimer(Timer);

pub struct WebSocketPlugin;

impl Plugin for WebSocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WsPlugin)
            .insert_resource(GenerateEnergyTimer(Timer::from_seconds(
                GENERATE_ENERGY_INTERVAL,
                TimerMode::Repeating,
            )))
            .add_systems(Startup, startup_socket_listener)
            .add_systems(
                Update,
                (
                    assign_client_to_socket,
                    receive_message,
                    generate_energy.after(receive_message),
                ),
            );
    }
}

fn startup_socket_listener(listener: Res<WsListener>) {
    listener.listen("127.0.0.1:8081");
}

fn assign_client_to_socket(
    mut commands: Commands,
    connections: Query<(Entity, &WsConnection), Added<WsConnection>>,
) {
    for (entity, conn) in connections.iter() {
        let player_bundle: PlayerBundle = PlayerBundle::new();
        let name = player_bundle.name.0.clone();
        commands.entity(entity).insert(player_bundle);
        conn.send(WSMessage::binary(server_stat_event::buffer(
            connections.iter().count() as u16,
        )));
        log::info!("Client connected: {:?}", name);
    }
}

fn receive_message(
    mut commands: Commands,
    mut connections: Query<(Entity, &WsConnection, &Name, &mut Energy)>,
) {
    let count = connections.iter().count();
    for (entity, conn, name, mut energy) in connections.iter_mut() {
        loop {
            match conn.receive() {
                Ok(message) => {
                    if energy.current <= 0 {
                        log::info!("{} has 0 energy left", name.0);
                        conn.send(WSMessage::text("0 energy left, closing connection"));
                        commands.entity(entity).despawn();
                        continue;
                    }
                    match message {
                        WSMessage::Text(data) => {
                            log::info!("Text Message: {}", &data)
                        }
                        WSMessage::Binary(data) => {
                            let message = flatbuffers::root::<Message>(&data).unwrap();
                            match message.message_type() {
                                MessageType::RequestGetServer => {
                                    energy.update(-1);
                                    conn.send(WSMessage::binary(server_stat_event::buffer(
                                        count as u16, //TODO: lets create a game state resource and use that for connection counts
                                    )));
                                    conn.send(WSMessage::text(
                                        format!("{} energy left", energy.current).as_str(),
                                    ));
                                }
                                MessageType::NoOp => {
                                    energy.update(-3);
                                    conn.send(WSMessage::text("You spent 3 energy calling NoOp"));
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}

fn generate_energy(
    time: Res<Time>,
    mut timer: ResMut<GenerateEnergyTimer>,
    mut players: Query<(&mut Energy, &Name, &WsConnection)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut energy, name, conn) in players.iter_mut() {
            let delta = energy.generation_sec;
            let current = energy.current;

            if current == energy.capacity {
                continue;
            }

            energy.update(delta);
            conn.send(WSMessage::Text(format!("+1 energy to {}", current + 1)));
            log::info!("{} has {} energy left", name.0, energy.current)
        }
    }
}
