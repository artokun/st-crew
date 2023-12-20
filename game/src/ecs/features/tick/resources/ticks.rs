use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Ticks {
    target_ticks_per_second: u8,
    current_ticks_per_second: f32,

    current_tick: u64,

    tick_timer: Timer,
}

impl Ticks {
    pub fn from_hz(target_ticks_per_second: u8) -> Self {
        Self {
            target_ticks_per_second,
            current_ticks_per_second: 0.0,

            current_tick: 0,

            tick_timer: Timer::new(
                Duration::from_secs_f32(1.0 / target_ticks_per_second as f32),
                TimerMode::Repeating,
            ),
        }
    }

    pub const fn target_ticks_per_second(&self) -> u8 {
        self.target_ticks_per_second
    }

    pub fn target_tick_duration(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.target_ticks_per_second as f32)
    }

    pub const fn current_ticks_per_second(&self) -> f32 {
        self.current_ticks_per_second
    }

    pub const fn current_tick(&self) -> u64 {
        self.current_tick
    }

    pub fn current_tick_duration(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.current_ticks_per_second)
    }

    /// Calculate the approximate amount of ticks until the given duration has passed.
    ///
    /// This can not realistically be considered a "real" ETA, as it does not take into account
    /// the actual current tick rate.
    pub fn ticks_until(&self, duration: Duration) -> u64 {
        (self.target_ticks_per_second as f32 * duration.as_secs_f32()).ceil() as u64
    }

    /// Calculate the approximate amount of time has passed until the given amount of ticks.
    ///
    /// This can not realistically be considered a "real" ETA, as it does not take into account
    /// the actual current tick rate.
    pub fn approximate_time_until(&self, ticks: f32) -> Duration {
        Duration::from_secs_f32(ticks / (self.target_ticks_per_second as f32))
    }

    /// Calculate the approximate amount of real time that will pass until the given amount of
    /// "game time" has passed.
    ///
    /// This is not necessarily accurate if the current tick rate is fluctuating.
    pub fn approximate_real_time_until(&self, duration: Duration) -> Duration {
        Duration::from_secs_f32(duration.as_secs_f32() / self.current_ticks_per_second)
    }

    pub(in crate::ecs::features::tick) fn tick(&mut self, delta: Duration) -> bool {
        self.tick_timer.tick(delta);

        if !self.tick_timer.just_finished() {
            false
        } else {
            self.current_tick += 1;

            true
        }
    }
}
