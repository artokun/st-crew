use bevy::prelude::*;

#[derive(Resource)]
pub struct Ticks(pub u64);

impl Ticks {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn tick(&mut self) {
        self.0 += 1;
    }
}
