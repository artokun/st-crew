use bevy::{prelude::*, utils::Uuid};

#[derive(Component)]
pub struct ClientComponent {
    pub uuid: Uuid,
    pub name: String,
    pub energy: i32,
    pub energy_capacity: i32,
    pub energy_generation_sec: i32,
    pub unit_capacity: i32,
}
