use bevy::ecs::schedule::SystemSet;

mod attach_energy;
mod energy_regen;

pub use attach_energy::*;
pub use energy_regen::*;

/// A system set that runs all energy-regeneration systems.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct RegenerateEnergy;
