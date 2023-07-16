use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;

pub(super) trait Behavior {
    fn update(&mut self, shared_state: &mut SharedState, stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>>;
    fn is_disposed(&self) -> bool { false }
}

