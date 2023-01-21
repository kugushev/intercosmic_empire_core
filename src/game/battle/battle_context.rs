use glam::Vec3;
use crate::game::battle::ecs_context::EcsContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_view_model::BattleViewModel;
use interoptopus::ffi_type;
use crate::game::battle::components::translation::Translation;
use crate::game::battle::models::warp_gate::WarpGate;
use crate::game::core::models::stellar_system::{StellarSystem};

#[ffi_type(opaque)]
pub struct BattleContext {
    pub(crate) ecs: EcsContext,
    pub(crate) warp_gates: Vec<WarpGate>,
}

impl BattleContext {
    pub(crate) fn new(battle_parameters: BattleParameters, stellar_system: StellarSystem) -> BattleContext {
        let mut ecs = EcsContext::new();

        ecs.world.insert_resource(battle_parameters);
        ecs.world.insert_resource(stellar_system);
        ecs.world.insert_resource(BattleViewModel::default());

        // todo: remove this test code
        ecs.world.spawn(
            Translation { position: Vec3::new(1.0, 2.0, 3.0) }
        );

        BattleContext {
            ecs,
            warp_gates: vec![],
        }
    }

    pub(crate) fn get_view_model(&self) -> &BattleViewModel {
        self.ecs.world.resource::<BattleViewModel>()
    }
    pub(crate) fn get_stellar_system(&self) -> &StellarSystem {
        self.ecs.world.resource::<StellarSystem>()
    }
    pub(crate) fn open_warp_gate(&mut self, warp_gate: WarpGate) {
        // todo: assert distance between planet and other warp gates
        self.warp_gates.push(warp_gate)
    }
}