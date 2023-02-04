use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use crate::game::battle::models::battle_state::BattleState;
use crate::game::battle::utils::game_time::GameTime;
use crate::game::battle::utils::interop_logger::InteropLogger;
use crate::game::core::models::stellar_system::{Production, StellarSystem};

pub(crate) fn stellar_production_cycle(mut stellar_system: ResMut<StellarSystem>,
                                       mut battle_state: ResMut<BattleState>,
                                       time: Res<GameTime>,
                                       mut logger: ResMut<InteropLogger>) {
    let delta_time = time.delta_time;

    if logger.trace_enabled {
        logger.log(format!("Start Production Cycle, deltaTime={delta_time}."));
    }

    for planet in stellar_system.planets.iter_mut() {
        increment_production(&mut planet.current_product, &planet.info.production, &delta_time);

        if logger.trace_enabled {
            let planet_id = planet.info.id.0;
            let new_product = &planet.current_product;
            logger.log(format!("New Product for planet {planet_id} is {new_product}"));
        }
    }

    for warp_gate in battle_state.warp_gates.iter_mut() {
        increment_production(&mut warp_gate.current_product, &warp_gate.production, &delta_time);

        if logger.trace_enabled {
            let new_product = &warp_gate.current_product;
            logger.log(format!("New Product for warp gate is {new_product}"));
        }
    }
}

fn increment_production(current_product: &mut f32, production: &Production, delta_time: &f32) {
    let new_product = *current_product + production.amount_per_second * delta_time;
    *current_product = new_product.min(production.max_product);
}