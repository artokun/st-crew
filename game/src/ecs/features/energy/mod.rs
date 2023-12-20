use bevy::prelude::*;

use crate::ecs::features::tick::TickUpdate;

use super::players::SpawnPlayer;

mod components;
mod socket_events;
mod systems;

pub use components::Energy;
pub use systems::RegenerateEnergy;

pub struct EnergyPlugin;

impl Plugin for EnergyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, systems::attach_player_energy.after(SpawnPlayer))
            .add_systems(
                TickUpdate,
                systems::do_energy_regen.in_set(RegenerateEnergy),
            )
            .add_systems(PostUpdate, systems::sync_player_energy);
    }
}
