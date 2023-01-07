use bevy_ecs::prelude::*;
use crate::game::battle::models::battle_view_model::BattleViewModel;
use crate::game::battle::systems::test_system::test_system;
use crate::game::battle::systems::view_sync::view_sync_translation;
use crate::game::battle::utils::game_time::GameTime;

pub(crate) struct EcsContext {
    pub(super) world: World,
    schedule: Schedule,
}

impl EcsContext {
    pub(crate) fn new(view_model: BattleViewModel) -> EcsContext {
        let mut world = World::new();
        world.insert_resource(GameTime::default());
        world.insert_resource(view_model);

        let mut schedule = Schedule::default();

        #[derive(StageLabel)]
        pub struct UpdateLabel;

        let stage = SystemStage::single_threaded()
            .with_system(test_system)
            .with_system(view_sync_translation);

        schedule.add_stage(UpdateLabel, stage);

        EcsContext {
            world,
            schedule,
        }
    }

    pub(crate) fn update(&mut self, delta_time: f32) {
        let mut time = self.world.resource_mut::<GameTime>();
        time.delta_time = delta_time;

        self.schedule.run(&mut self.world);
    }
}