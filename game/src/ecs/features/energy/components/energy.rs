use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Energy {
    pub current: u16,
    pub capacity: u16,
}

#[derive(Component)]
pub struct EnergyRegeneration {
    pub amount: u16,

    pub interval: f32,
    pub last_regen_at: f32,
}
