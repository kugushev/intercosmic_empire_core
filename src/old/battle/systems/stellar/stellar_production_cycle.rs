use bevy_ecs::prelude::{NonSend, Res};
use bevy_ecs::system::ResMut;
use crate::old::battle::models::battle_state::BattleState;
use crate::old::battle::utils::game_time::GameTime;
use crate::old::core::models::faction::Faction;
use crate::old::core::models::stellar_system::{Production, StellarSystem};
use crate::old::utils::interop_logger::LoggerRef;
use crate::trace_old;

pub(crate) fn stellar_production_cycle(mut stellar_system: ResMut<StellarSystem>,
                                       mut battle_state: ResMut<BattleState>,
                                       time: Res<GameTime>,
                                       logger: NonSend<LoggerRef>) {
    let delta_time = time.delta_time;

   trace_old!(logger, format!("Start Production Cycle, deltaTime={delta_time}."));

    for planet in stellar_system.planets.iter_mut() {
        if planet.faction == Faction::Grey || planet.under_siege {
            continue;
        }

        increment_production(&mut planet.current_product, &planet.info.production, &delta_time);

        trace_old!(logger, {
            let planet_id = planet.info.id.0;
            let new_product = &planet.current_product;
            format!("New Product for planet {planet_id} is {new_product}")
        });
    }

    for warp_gate in battle_state.warp_gates.iter_mut() {
        increment_production(&mut warp_gate.current_product, &warp_gate.production, &delta_time);

        trace_old!(logger, {
            let new_product = &warp_gate.current_product;
            format!("New Product for warp gate is {new_product}")
        });
    }
}

fn increment_production(current_product: &mut f32, production: &Production, delta_time: &f32) {
    let new_product = *current_product + production.amount_per_second * delta_time;
    *current_product = new_product.min(production.max_product);
}