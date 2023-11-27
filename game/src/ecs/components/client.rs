use bevy::prelude::*;

#[derive(Component)]
pub struct ClientComponent {
    pub uuid: u128,
    pub name: String,
    pub energy: i32,
    pub energy_capacity: i32,
    pub energy_generation_sec: i32,
    pub unit_capacity: i32,
}
