use std::{
    cmp::Ordering,
    ops::{Div, Mul},
};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Speed(f32);

impl Speed {
    pub const fn per_second(per_second: f32) -> Self {
        Self(per_second)
    }

    pub fn into_inner(self) -> f32 {
        self.0
    }
}

impl Mul<f32> for Speed {
    type Output = Speed;

    fn mul(self, rhs: f32) -> Self::Output {
        Speed::per_second(self.0 * rhs)
    }
}

impl Mul<Speed> for f32 {
    type Output = Speed;

    fn mul(self, rhs: Speed) -> Self::Output {
        Speed::per_second(self * rhs.0)
    }
}

impl Mul<Speed> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Speed) -> Self::Output {
        Vec3::new(self.x * rhs.0, self.y * rhs.0, self.z * rhs.0)
    }
}

impl Div<Speed> for f32 {
    type Output = Speed;

    fn div(self, rhs: Speed) -> Self::Output {
        Speed::per_second(self / rhs.0)
    }
}

impl Div<f32> for Speed {
    type Output = Speed;

    fn div(self, rhs: f32) -> Self::Output {
        Speed::per_second(self.0 / rhs)
    }
}

impl Div<Speed> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Speed) -> Self::Output {
        Vec3::new(self.x / rhs.0, self.y / rhs.0, self.z / rhs.0)
    }
}

impl PartialEq<f32> for Speed {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<Speed> for f32 {
    fn eq(&self, other: &Speed) -> bool {
        *self == other.0
    }
}

impl PartialOrd<f32> for Speed {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<Speed> for f32 {
    fn partial_cmp(&self, other: &Speed) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}
