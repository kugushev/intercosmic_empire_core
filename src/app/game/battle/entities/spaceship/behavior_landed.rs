use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::behavior_disposed::BehaviorDisposed;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::battle::traits::Spawner;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::trace;

pub struct BehaviorLanded;

const REINFORCE_PRODUCT_PER_SECOND: f32 = 10.0;
const DAMAGE_PRODUCT_PER_SECOND: f32 = 10.0;

impl BehaviorLanded {
    fn handle_reinforcements(&self, finish: &mut dyn Spawner, shared_state: &mut SharedState, delta: DeltaTime) {
        let mut amount = REINFORCE_PRODUCT_PER_SECOND * delta.seconds();
        if shared_state.product < amount {
            amount = shared_state.product;
        }

        shared_state.product -= amount;
        finish.increase_product(amount, true);
    }

    fn handle_land_fight(&self, finish: &mut dyn Spawner, shared_state: &mut SharedState, delta: DeltaTime) {
        let mut amount = DAMAGE_PRODUCT_PER_SECOND * delta.seconds();
        if shared_state.product < amount {
            amount = shared_state.product;
        }

        shared_state.product -= amount;
        finish.receive_damage(amount, shared_state.faction);
    }
}

impl BehaviorLanded {}

impl Behavior for BehaviorLanded {
    fn update(&mut self, shared_state: &mut SharedState, stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        if shared_state.product <= 0.0 {
            return Some(Box::new(BehaviorDisposed));
        }

        let finish_id = *shared_state.route.finish_id();
        let finish = match stellar_system.find_spawner(finish_id, true) {
            Err(error) => {
                trace!(logger, format!("Unable to find spawner id:{} err:{}", finish_id, error));
                return None;
            }
            Ok(body) => { body }
        };

        if shared_state.faction == finish.get_belonging() {
            self.handle_reinforcements(finish, shared_state, delta);
        } else {
            self.handle_land_fight(finish, shared_state, delta);
        }

        None
    }
}