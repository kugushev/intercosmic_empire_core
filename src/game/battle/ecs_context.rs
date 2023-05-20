use bevy_ecs::prelude::*;
use crate::ffi_models::FFILog;
use crate::game::battle::systems::spaceships::spaceship_movement::spaceship_move;
use crate::game::battle::systems::stellar::stellar_production_cycle::stellar_production_cycle;
use crate::game::battle::systems::views::spaceship_view_sync::spaceship_view_sync;
use crate::game::battle::utils::game_time::GameTime;
use crate::game::battle::utils::interop_logger::InteropLogger;

pub(crate) struct EcsContext {
    pub(super) world: World,
    schedule: Schedule,
}

impl EcsContext {
    pub(crate) fn new() -> EcsContext {
        let mut world = World::new();
        world.insert_resource(GameTime::default());
        world.insert_resource(InteropLogger::default());

        let mut schedule = Schedule::default();

        #[derive(StageLabel)]
        pub struct InputLabel;
        let input_stage = SystemStage::single_threaded();
        schedule.add_stage(InputLabel, input_stage);

        #[derive(StageLabel)]
        pub struct MainLabel;
        let main_stage = SystemStage::single_threaded()
            .with_system(stellar_production_cycle);
        schedule.add_stage_after(InputLabel, MainLabel, main_stage);

        #[derive(StageLabel)]
        pub struct ViewSyncLabel;
        let view_sync_stage = SystemStage::single_threaded()
            .with_system(spaceship_view_sync)
            .with_system(spaceship_move);
        schedule.add_stage_after(MainLabel, ViewSyncLabel, view_sync_stage);

        EcsContext {
            world,
            schedule,
        }
    }

    pub(crate) fn update(&mut self, delta_time: f32, log: FFILog) {
        let mut time = self.world.resource_mut::<GameTime>();
        time.delta_time = delta_time;

        let mut interop_logger = self.world.resource_mut::<InteropLogger>();
        interop_logger.ffi_log = log;

        if interop_logger.trace_enabled {
            interop_logger.log("Do Update".to_string());
        }

        self.schedule.run(&mut self.world);
    }
}