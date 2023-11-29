use bevy::{prelude::*, utils::Uuid};

use crate::ecs::plugins::websocket::ConnectionId;

#[derive(Component)]
pub struct Player(Uuid);

#[derive(Component)]
pub struct Energy {
    pub current: i32,
    pub capacity: i32,
    pub generation_sec: i32,
}

#[derive(Component)]
pub struct UnitCapacity {
    pub capacity: i32,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}

// Bundle
#[derive(Bundle)]
pub struct PlayerBundle {
    pub connection_id: ConnectionId,
    pub player: Player,
    pub energy: Energy,
    pub unit_capacity: UnitCapacity,
    pub name: Name,
}

impl PlayerBundle {
    pub fn new(connection_id: ConnectionId) -> Self {
        let uuid = Uuid::new_v4();
        let name = format!("player-{}", &uuid.to_string()[..8]);

        Self {
            player: Player(uuid),
            connection_id,
            energy: Energy {
                current: 10,
                capacity: 10,
                generation_sec: 1,
            },
            unit_capacity: UnitCapacity { capacity: 1 },
            name: Name { name },
        }
    }

    pub fn increment_energy(&mut self) -> &Self {
        self.energy.current += self.energy.generation_sec;
        if self.energy.current > self.energy.capacity {
            self.energy.current = self.energy.capacity;
        }
        self
    }

    pub fn decrement_energy(&mut self, amount: i32) -> &Self {
        self.energy.current -= amount;
        if self.energy.current < 0 {
            self.energy.current = 0;
        }
        self
    }

    pub fn update_name(&mut self, name: String) -> &Self {
        self.name.name = name;
        self
    }
}
