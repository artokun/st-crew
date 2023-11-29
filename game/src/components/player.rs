use bevy::{prelude::*, utils::Uuid};

#[derive(Component)]
pub struct Player(Uuid);

#[derive(Component)]
pub struct UnitCapacity(pub i32);

#[derive(Component, Debug)]
pub struct PlayerName(pub String);

#[derive(Component)]
pub struct Energy {
    pub current: i32,
    pub capacity: i32,
    pub generation_sec: i32,
}

impl Energy {
    pub fn update(&mut self, delta: i32) -> &Self {
        self.current = (self.current + delta).clamp(0, self.capacity);
        self
    }
}

// Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub energy: Energy,
    pub unit_capacity: UnitCapacity,
    pub name: PlayerName,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        let name = format!("player-{}", &uuid.to_string()[..8]);

        Self {
            marker: Player(uuid),
            energy: Energy {
                current: 10,
                capacity: 10,
                generation_sec: 1,
            },
            unit_capacity: UnitCapacity(1),
            name: PlayerName(name),
        }
    }
}
