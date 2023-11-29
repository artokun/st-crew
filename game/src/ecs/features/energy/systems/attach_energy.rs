use bevy::{log, prelude::*};

use crate::ecs::features::{
    energy::{components::EnergyRegeneration, Energy},
    players::Player,
};

pub fn attach_player_energy(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<Entity, Added<Player>>,
) {
    for entity in query.iter() {
        log::info!("attaching energy components to entity: {:?}", entity);

        commands
            .entity(entity)
            .insert(Energy {
                current: 5,
                capacity: 10,
            })
            .insert(EnergyRegeneration {
                amount: 1,

                interval: 1.0,
                last_regen_at: time.elapsed_seconds(),
            });
    }
}
