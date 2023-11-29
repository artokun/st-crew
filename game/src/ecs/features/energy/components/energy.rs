use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Energy {
    pub current: u16,
    pub capacity: u16,
}

impl Energy {
    pub fn spend(&mut self, amount: u16) -> Result<(), String> {
        if self.current >= amount {
            self.current -= amount;
            Ok(())
        } else {
            Err("Not enough energy".to_owned())
        }
    }
}

#[derive(Component)]
pub struct EnergyRegeneration {
    pub amount: u16,

    pub interval: f32,
    pub last_regen_at: f32,
}
