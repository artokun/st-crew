use bevy::{
    app::{App, Plugin, PreUpdate, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};
use st_commander::{CommanderServerExt, ReceiveNetworkMessages};

mod commands;
mod models;
mod socket_events;
mod systems;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.register_command::<commands::get_server_info::GetServerInfoCommand>()
            .get_endpoint(commands::get_server_info::route_get_server_info)
            .add_systems(Update, commands::get_server_info::on_server_info_command);

        app.add_systems(Startup, systems::startup_socket_listener)
            .add_systems(
                PreUpdate,
                systems::log_connection_events.after(ReceiveNetworkMessages),
            )
            .add_systems(
                Update,
                (
                    systems::send_clients_connected_on_join,
                    // systems::handle_message,
                ),
            );
    }
}
