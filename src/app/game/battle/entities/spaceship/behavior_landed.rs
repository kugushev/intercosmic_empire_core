use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;

pub struct BehaviorLanded {

}

impl BehaviorLanded {
    
}

impl Behavior for BehaviorLanded {
    fn update(&mut self, shared_state: &mut SharedState, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        todo!("If the same faction increase product with high speed, if not FIGHT (add current product to shared state and decrease in fight)")
    }
}