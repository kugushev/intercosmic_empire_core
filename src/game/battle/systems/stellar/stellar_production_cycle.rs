use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use crate::game::battle::utils::game_time::GameTime;
use crate::game::core::models::stellar_system::StellarSystem;

pub(crate) fn stellar_production_cycle(mut stellar_system: ResMut<StellarSystem>, time: Res<GameTime>) {
    for planet in stellar_system.planets.iter_mut() {
        let new_product = planet.current_product + planet.info.production.amount_per_second * time.delta_time;
        planet.current_product = new_product.min(planet.info.production.max_product);
    }
}