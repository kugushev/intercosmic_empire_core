use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;

pub(super) struct BehaviorDisposed;

impl Behavior for BehaviorDisposed {
    fn update(&mut self, _shared_state: &mut SharedState, _delta: DeltaTime, _logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        None
    }

    fn is_disposed(&self) -> bool { true }
}