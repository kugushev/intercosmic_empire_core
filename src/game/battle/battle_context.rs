use glam::Vec3;
use crate::game::battle::ecs_context::EcsContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_view_model::BattleViewModel;
use interoptopus::ffi_type;
use crate::game::battle::components::translation::Translation;

#[ffi_type(opaque)]
pub struct BattleContext {
    pub(crate) ecs: EcsContext
}

impl BattleContext {
    pub(crate) fn new() -> BattleContext {
        BattleContext {
            ecs: EcsContext::new(BattleViewModel::default())
        }
    }

    pub(crate) fn init(&mut self, battle_parameters: BattleParameters) {
        // todo: drop the current view state and entities
        self.ecs.world.insert_resource(battle_parameters);

        // todo: remove this test code
        self.ecs.world.spawn(
            Translation { position: Vec3::new(1.0, 2.0, 3.0) }
        );
    }

    pub(crate) fn get_view_model(&mut self) -> &BattleViewModel {
        self.ecs.world.resource::<BattleViewModel>()
    }
}