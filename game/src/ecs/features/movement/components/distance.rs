use std::ops::{Div, Mul};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ecs::features::{movement::Speed, tick::TickDuration};

#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Distance(f32);

impl Distance {
    // We should really determine what 1 unit of distance is in the game world
    pub const fn new(units: f32) -> Self {
        Self(units)
    }

    pub fn between_positions(a: Vec3, b: Vec3) -> Self {
        Self((a - b).length())
    }

    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl Div<Speed> for Distance {
    type Output = TickDuration;

    fn div(self, rhs: Speed) -> Self::Output {
        TickDuration::new(self.0 / rhs.into_inner())
    }
}

impl Mul<Distance> for f32 {
    type Output = Distance;

    fn mul(self, rhs: Distance) -> Self::Output {
        Distance::new(self * rhs.0)
    }
}

impl Mul<f32> for Distance {
    type Output = Distance;

    fn mul(self, rhs: f32) -> Self::Output {
        Distance::new(self.0 * rhs)
    }
}
