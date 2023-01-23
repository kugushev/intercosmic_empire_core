use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use crate::game::battle::utils::game_time::GameTime;
use crate::game::battle::utils::interop_logger::InteropLogger;
use crate::game::core::models::stellar_system::StellarSystem;

pub(crate) fn stellar_production_cycle(mut stellar_system: ResMut<StellarSystem>, time: Res<GameTime>,
                                       mut logger: ResMut<InteropLogger>) {
    let delta_time = time.delta_time;
    //logger.log(format!("Start Production Cycle, deltaTime={delta_time}."));
    for planet in stellar_system.planets.iter_mut() {
        let new_product = planet.current_product + planet.info.production.amount_per_second * delta_time;
        planet.current_product = new_product.min(planet.info.production.max_product);

        //let planet_id = planet.info.id.0;
        //logger.log(format!("New Product for planet {planet_id} is {new_product}"));
    }
}