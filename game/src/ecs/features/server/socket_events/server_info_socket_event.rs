use serde::Serialize;
use st_commander::event::SocketEvent;
use utoipa::ToSchema;

use crate::ecs::features::server::models::ServerInfo;

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerInfoSocketEvent(pub ServerInfo);

impl SocketEvent for ServerInfoSocketEvent {
    const NAME: &'static str = "server_info";
}
