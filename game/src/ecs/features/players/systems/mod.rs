use bevy::prelude::*;

mod update_connected_players;

pub use update_connected_players::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpawnPlayer;
