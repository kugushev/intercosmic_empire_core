mod behavior;
mod shared_state;
mod behavior_move;
mod behavior_departure;
mod behavior_arrival;
mod behavior_disposed;
mod behavior_attack;
mod behavior_landed;

use interoptopus::{ffi_surrogates, ffi_type};
use crate::app::game::battle::entities::route::Route;
use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::behavior_departure::BehaviorDeparture;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::translation::FFITranslation;
use crate::app::utils::zero;

pub const EMPTY_SPACESHIP_VM: u64 = 0;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct SpaceshipViewModel {
    pub id: u64,
    pub translation: FFITranslation,
    pub faction: Faction,
    pub mark: SpaceshipMark,
}

impl SpaceshipViewModel {
    pub fn empty() -> Self {
        Self {
            id: EMPTY_SPACESHIP_VM,
            translation: zero(),
            faction: zero(),
            mark: zero(),
        }
    }
}

pub struct Spaceship {
    id: u64,
    behavior: Box<dyn Behavior>,
    state: SharedState,
}

impl Spaceship {
    pub fn new(id: u64, mark: SpaceshipMark, route: Route, faction: Faction) -> Self {
        let state = SharedState::new(mark, route, faction);
        let behavior = Box::new(BehaviorDeparture::new(&state));
        Self { id, state, behavior }
    }

    pub fn get_vm(&self) -> SpaceshipViewModel {
        SpaceshipViewModel {
            id: self.id,
            translation: (&self.state.translation).into(),
            faction: self.state.faction,
            mark: self.state.mark,
        }
    }

    pub fn update(&mut self, stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) {
        let shared_state = &mut self.state;
        let new_behavior = self.behavior.update(shared_state, stellar_system, delta, logger);
        if let Some(behavior) = new_behavior {
            self.behavior = behavior;
        }
    }

    pub fn disposed(&self) -> bool { self.behavior.is_disposed() }
}

