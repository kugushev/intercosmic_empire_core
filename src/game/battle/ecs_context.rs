use bevy_ecs::prelude::*;
use crate::game::battle::systems::test_system::test_system;
use crate::game::battle::systems::view_sync::view_sync_translation;
use crate::game::battle::utils::game_time::GameTime;

pub(crate) struct EcsContext {
    pub(super) world: World,
    schedule: Schedule,
}

impl EcsContext {
    pub(crate) fn new() -> EcsContext {
        let mut world = World::new();
        world.insert_resource(GameTime::default());

        let mut schedule = Schedule::default();

        #[derive(StageLabel)]
        pub struct MainLabel;

        let main_stage = SystemStage::single_threaded()
            .with_system(test_system);

        schedule.add_stage(MainLabel, main_stage);

        #[derive(StageLabel)]
        pub struct ViewSyncLabel;
        let view_sync_stage = SystemStage::single_threaded()
            .with_system(view_sync_translation);
        schedule.add_stage_after(MainLabel, ViewSyncLabel, view_sync_stage);

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