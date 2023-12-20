use serde::Serialize;
use st_commander::event::SocketEvent;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionWithEta {
    pub uuid: String,
    pub origin: (f32, f32),
    pub destination: (f32, f32),
    pub current_tick: u64,
    pub arrival_tick: u64,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MovementChangedSocketEvent {
    pub droid_positions: Vec<PositionWithEta>,
}

impl SocketEvent for MovementChangedSocketEvent {
    const NAME: &'static str = "movement_changed";
}
