use std::time::Duration;

use bevy::prelude::*;

use crate::ecs::features::tick::{models::Tick, TickDuration};

#[derive(Resource)]
pub struct TickTimer {
    target_ticks_per_second: u8,

    // TODO: actually update this value
    current_ticks_per_second: f32,

    current_tick: Tick,

    tick_timer: Timer,
}

impl TickTimer {
    pub fn from_hz(target_ticks_per_second: u8) -> Self {
        Self {
            target_ticks_per_second,
            current_ticks_per_second: target_ticks_per_second as f32,

            current_tick: Tick::new(0),

            tick_timer: Timer::new(
                Duration::from_secs_f32(1.0 / target_ticks_per_second as f32),
                TimerMode::Repeating,
            ),
        }
    }

    pub const fn target_ticks_per_second(&self) -> u8 {
        self.target_ticks_per_second
    }

    pub fn target_tick_time(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.target_ticks_per_second as f32)
    }

    pub const fn current_ticks_per_second(&self) -> f32 {
        self.current_ticks_per_second
    }

    pub fn current_tick_time(&self) -> Duration {
        Duration::from_secs_f32(1.0 / self.current_ticks_per_second)
    }

    pub const fn current_tick(&self) -> Tick {
        self.current_tick
    }

    /// Calculate the target amount of ticks that will pass until the given amount of time
    /// has elapsed.
    ///
    /// This can not realistically be considered a "real" ETA, as it does not take into account
    /// the actual current tick rate.
    pub fn target_ticks_until(&self, duration: Duration) -> TickDuration {
        TickDuration::new(self.target_ticks_per_second as f32 * duration.as_secs_f32())
    }

    /// Calculate the target amount of time that will pass until the given amount of ticks
    /// have elapsed.
    ///
    /// This can not realistically be considered a "real" ETA, as it does not take into account
    /// the actual current tick rate.
    pub fn target_time_until(&self, tick_duration: TickDuration) -> Duration {
        Duration::from_secs_f32(tick_duration.into_inner() / (self.target_ticks_per_second as f32))
    }

    /// Approximate the amount of ticks that will pass until the given amount of time has elapsed.
    ///
    /// This is not necessarily accurate if the current tick rate is fluctuating.
    pub fn current_ticks_until(&self, duration: Duration) -> TickDuration {
        TickDuration::new(self.current_ticks_per_second * duration.as_secs_f32())
    }

    /// Approximate the amount of time that will pass until the given amount of ticks have elapsed.
    ///
    /// This is not necessarily accurate if the current tick rate is fluctuating.
    pub fn current_time_until(&self, tick_duration: TickDuration) -> Duration {
        Duration::from_secs_f32(tick_duration.into_inner() / self.current_ticks_per_second)
    }

    /// Calculate the current time dialation, which is the ratio between the current tick rate and
    /// the target tick rate.
    pub fn time_dialation(&self) -> f32 {
        self.current_ticks_per_second / self.target_ticks_per_second as f32
    }

    /// Calculate the approximate amount of "game time" that will pass until the given amount of
    /// "real time" has passed.
    ///
    /// This is not necessarily accurate if the current tick rate is fluctuating.
    pub fn approximate_game_time(&self, real_duration: Duration) -> Duration {
        Duration::from_secs_f32(real_duration.as_secs_f32() * self.time_dialation())
    }

    /// Calculate the approximate amount of "real time" that will pass until the given amount of
    /// "game time" has passed.
    ///
    /// This is not necessarily accurate if the current tick rate is fluctuating.
    pub fn approximate_real_time(&self, game_duration: Duration) -> Duration {
        Duration::from_secs_f32(game_duration.as_secs_f32() / self.time_dialation())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_ticks_until() {
        let tick_timer = TickTimer::from_hz(10);

        assert_eq!(
            tick_timer.target_ticks_until(Duration::from_secs(1)),
            TickDuration::new(10.0)
        );
        assert_eq!(
            tick_timer.target_ticks_until(Duration::from_secs_f32(0.5)),
            TickDuration::new(5.0)
        );
    }

    #[test]
    fn test_target_time_until() {
        let tick_timer = TickTimer::from_hz(10);

        assert_eq!(
            tick_timer.target_time_until(TickDuration::new(10.0)),
            Duration::from_secs(1)
        );
        assert_eq!(
            tick_timer.target_time_until(TickDuration::new(5.0)),
            Duration::from_secs_f32(0.5)
        );
    }

    #[test]
    fn test_current_ticks_until() {
        let mut tick_timer = TickTimer::from_hz(10);

        tick_timer.current_ticks_per_second = 5.0;

        assert_eq!(
            tick_timer.current_ticks_until(Duration::from_secs(1)),
            TickDuration::new(5.0)
        );
        assert_eq!(
            tick_timer.current_ticks_until(Duration::from_secs_f32(0.5)),
            TickDuration::new(2.5)
        );
    }

    #[test]
    fn test_current_time_until() {
        let mut tick_timer = TickTimer::from_hz(10);

        tick_timer.current_ticks_per_second = 5.0;

        assert_eq!(
            tick_timer.current_time_until(TickDuration::new(5.0)),
            Duration::from_secs(1)
        );
        assert_eq!(
            tick_timer.current_time_until(TickDuration::new(2.5)),
            Duration::from_secs_f32(0.5)
        );
    }

    #[test]
    fn test_time_dialation() {
        let mut tick_timer = TickTimer::from_hz(10);

        tick_timer.current_ticks_per_second = 5.0;

        assert_eq!(tick_timer.time_dialation(), 0.5);
    }

    #[test]
    fn test_approximate_game_time() {
        let mut tick_timer = TickTimer::from_hz(10);

        tick_timer.current_ticks_per_second = 5.0;

        assert_eq!(
            tick_timer.approximate_game_time(Duration::from_secs(1)),
            Duration::from_secs_f32(0.5)
        );
        assert_eq!(
            tick_timer.approximate_game_time(Duration::from_secs_f32(0.5)),
            Duration::from_secs_f32(0.25)
        );
    }

    #[test]
    fn test_approximate_real_time() {
        let mut tick_timer = TickTimer::from_hz(10);

        tick_timer.current_ticks_per_second = 5.0;

        assert_eq!(
            tick_timer.approximate_real_time(Duration::from_secs(1)),
            Duration::from_secs_f32(2.0)
        );
        assert_eq!(
            tick_timer.approximate_real_time(Duration::from_secs_f32(0.5)),
            Duration::from_secs_f32(1.0)
        );
    }
}
