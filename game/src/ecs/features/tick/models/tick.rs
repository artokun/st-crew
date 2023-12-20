use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::ecs::features::tick::TickDuration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(u64);

impl Tick {
    pub(in crate::ecs::features::tick) const fn new(current_tick: u64) -> Self {
        Self(current_tick)
    }

    pub const fn into_inner(self) -> u64 {
        self.0
    }
}

impl Add<u64> for Tick {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<u64> for Tick {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl Add<TickDuration> for Tick {
    type Output = Tick;

    fn add(self, rhs: TickDuration) -> Self::Output {
        Tick::new((self.0 as f32 + rhs.into_inner()).ceil() as u64)
    }
}

impl Sub<u64> for Tick {
    type Output = Self;

    fn sub(self, rhs: u64) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<u64> for Tick {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl Mul<u64> for Tick {
    type Output = u64;

    fn mul(self, rhs: u64) -> Self::Output {
        self.0 * rhs
    }
}
