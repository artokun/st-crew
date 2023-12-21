use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::ecs::features::tick::Tick;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TickDuration(f32);

impl TickDuration {
    pub const fn new(num_ticks: f32) -> Self {
        Self(num_ticks)
    }

    pub const fn into_inner(self) -> f32 {
        self.0
    }
}

impl Add<Tick> for TickDuration {
    type Output = Self;

    fn add(self, rhs: Tick) -> Self::Output {
        Self(self.0 + rhs.into_inner() as f32)
    }
}

impl AddAssign<Tick> for TickDuration {
    fn add_assign(&mut self, rhs: Tick) {
        self.0 += rhs.into_inner() as f32;
    }
}

impl Sub<Tick> for TickDuration {
    type Output = Self;

    fn sub(self, rhs: Tick) -> Self::Output {
        Self(self.0 - rhs.into_inner() as f32)
    }
}

impl SubAssign<Tick> for TickDuration {
    fn sub_assign(&mut self, rhs: Tick) {
        self.0 -= rhs.into_inner() as f32;
    }
}
