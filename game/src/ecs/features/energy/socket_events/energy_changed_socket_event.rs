use serde::Serialize;
use st_commander::event::SocketEvent;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EnergyChangedSocketEvent {
    pub current: u16,
    pub capacity: u16,
}

impl SocketEvent for EnergyChangedSocketEvent {
    const NAME: &'static str = "energy_changed";
}
