use bevy::{
    ecs::schedule::{InternedScheduleLabel, ScheduleLabel},
    prelude::*,
};

use crate::ecs::features::tick::Ticks;

/// Runs first in the [`TickMain`] schedule.
///
/// See the [`TickMain`] schedule for details on how tick updates work.
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickFirst;

/// The schedule that contains logic that must run before [`TickUpdate`].
///
/// See the [`TickMain`] schedule for details on how tick updates work.
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickPreUpdate;

/// The schedule that contains most gameplay logic.
///
/// See the [`TickMain`] schedule for details on how tick updates work.
/// See the [`Main`] schedule for some details about how schedules are run.
///
/// NOTE: Reading events during the `Tick` schedule that were not created during
/// the same tick is not supported. This is because events are not guaranteed
/// to exist within bevy longer than 2 updates. Since ticks and updates are
/// separate, events created in any other schedule are not guaranteed to exist
/// during the `Tick` schedule.
///
/// However, reading events created within a tick by Bevy's [`Update`] schedule is
/// supported since events are guaranteed to exist for at least 2 updates.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickUpdate;

/// The schedule that runs after the [`TickUpdate`] schedule, for reacting
/// to changes made in the main update logic.
///
/// See the [`TickMain`] schedule for details on how tick updates work.
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickPostUpdate;

/// The schedule that runs last in [`TickMain`]
///
/// See the [`TickMain`] schedule for details on how fixed updates work.
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickLast;

/// The schedule that contains systems which only run after a period of time has elapsed.
///
/// The exclusive `run_tick_main_schedule` system runs this schedule.
/// This is run by the [`RunTickMain`] schedule.
///
/// Frequency of execution is configured by inserting `Time<Tick>` resource, 20 Hz by default.
/// See [this example](https://github.com/bevyengine/bevy/blob/latest/examples/time/time.rs).
///
/// See the [`Main`] schedule for some details about how schedules are run.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TickMain;

/// Defines the schedules to be run for the [`TickMain`] schedule, including
/// their order.
#[derive(Resource, Debug)]
pub struct TickMainScheduleOrder {
    /// The labels to run for the [`TickMain`] schedule (in the order they will be run).
    pub labels: Vec<InternedScheduleLabel>,
}

impl Default for TickMainScheduleOrder {
    fn default() -> Self {
        Self {
            labels: vec![
                TickFirst.intern(),
                TickPreUpdate.intern(),
                TickUpdate.intern(),
                TickPostUpdate.intern(),
                TickLast.intern(),
            ],
        }
    }
}

impl TickMainScheduleOrder {
    /// Adds the given `schedule` after the `after` schedule
    pub fn insert_after(&mut self, after: impl ScheduleLabel, schedule: impl ScheduleLabel) {
        let index = self
            .labels
            .iter()
            .position(|current| (**current).eq(&after))
            .unwrap_or_else(|| panic!("Expected {after:?} to exist"));
        self.labels.insert(index + 1, schedule.intern());
    }
}

impl TickMain {
    /// A system that runs the tick timestep's "main schedule"
    pub fn run_tick_main(world: &mut World) {
        let delta = world.resource::<Time<Virtual>>().delta();
        let mut ticks = world.resource_mut::<Ticks>();

        if !ticks.tick(delta) {
            return;
        }

        world.resource_scope(|world, order: Mut<TickMainScheduleOrder>| {
            for &label in &order.labels {
                let _ = world.try_run_schedule(label);
            }
        });
    }
}
