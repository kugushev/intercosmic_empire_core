use crate::game::battle::ecs_context::EcsContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_state::BattleState;
use interoptopus::ffi_type;
use crate::game::battle::components::stellar::warp_gate::WarpGate;
use crate::game::core::models::stellar_system::{StellarSystem};

#[ffi_type(opaque)]
pub struct BattleContext {
    pub(crate) ecs: EcsContext,
}

impl BattleContext {
    pub(crate) fn new(battle_parameters: BattleParameters, stellar_system: StellarSystem) -> BattleContext {
        let mut ecs = EcsContext::new();

        ecs.world.insert_resource(battle_parameters);
        ecs.world.insert_resource(stellar_system);
        ecs.world.insert_resource(BattleState::default());

        BattleContext {
            ecs
        }
    }

    pub(crate) fn get_view_model(&self) -> &BattleState {
        self.ecs.world.resource::<BattleState>()
    }
    pub(crate) fn get_stellar_system(&self) -> &StellarSystem {
        self.ecs.world.resource::<StellarSystem>()
    }
    pub(crate) fn open_warp_gate(&mut self, warp_gate: WarpGate) {
        // todo: assert distance between planet and other warp gates

        let mut view_state = self.ecs.world.resource_mut::<BattleState>();
        view_state.warp_gates.push(warp_gate);
    }
}