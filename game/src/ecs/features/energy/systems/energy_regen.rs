use bevy::{
    ecs::{
        query::Changed,
        system::{Query, Res},
    },
    log,
    time::Time,
};

use crate::ecs::{
    features::energy::components::{Energy, EnergyRegeneration},
    plugins::websocket::{ConnectionId, WsConnections, WsMessage},
};

pub fn do_energy_regen(time: Res<Time>, mut query: Query<(&mut Energy, &mut EnergyRegeneration)>) {
    for (mut energy, mut regen) in query.iter_mut() {
        let regen_delta = time.elapsed_seconds() - regen.last_regen_at;

        if regen_delta > regen.interval {
            regen.last_regen_at = time.elapsed_seconds();

            if energy.current < energy.capacity {
                // It's entirely possible that we got delayed and need to regen more than once
                let regen_amount =
                    ((regen_delta / regen.interval).floor() as u16).saturating_mul(regen.amount);

                energy.current = energy
                    .current
                    .saturating_add(regen_amount)
                    .clamp(0, energy.capacity);

                log::info!(
                    last_regen_at = ?regen.last_regen_at,
                    interval = ?regen.interval,
                    regen_amount = ?regen_amount,
                    new_amount = ?energy.current,
                    "energy regenerated"
                );
            }
        }
    }
}

pub fn sync_player_energy_regen(
    mut query: Query<(&ConnectionId, &Energy), Changed<Energy>>,
    connections: Res<WsConnections>,
) {
    for (connection_id, new_energy) in query.iter_mut() {
        let connection = connections.get(connection_id).expect("no connection found");

        connection
            .send_raw(WsMessage::Text(format!(
                "Your energy has changed! You now have {}/{}.",
                new_energy.current, new_energy.capacity
            )))
            .ok();
    }
}
