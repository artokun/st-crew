use bevy::{app::MainScheduleOrder, ecs::schedule::ExecutorKind, prelude::*};

use crate::ecs::features::tick::schedule::{TickMain, TickMainScheduleOrder};

mod models;
mod resources;
mod schedule;

pub use resources::*;
pub use schedule::{TickFirst, TickLast, TickPostUpdate, TickPreUpdate, TickUpdate};

pub struct TickPlugin;

const TICKS_PER_SECOND: u8 = 4;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        let mut tick_main_schedule = Schedule::new(TickMain);
        tick_main_schedule.set_executor_kind(ExecutorKind::MultiThreaded);

        let mut order = app.world.resource_mut::<MainScheduleOrder>();
        order.insert_after(StateTransition, TickMain);

        app.add_schedule(tick_main_schedule)
            .init_resource::<TickMainScheduleOrder>()
            .insert_resource(Ticks::from_hz(TICKS_PER_SECOND))
            .add_systems(TickMain, TickMain::run_tick_main);

        // An upcoming bevy update may add a way to persist events until certain schedules,
        // run. The current `master` branch as of now does the following:
        // app.init_resource::<EventUpdateSignal>()
        //     .add_systems(FixedPostUpdate, event_queue_update_system);
    }
}
