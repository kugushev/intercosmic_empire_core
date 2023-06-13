use bevy_ecs::prelude::*;
use crate::game::battle::systems::spaceships::spaceship_movement::spaceship_move;
use crate::game::battle::systems::stellar::stellar_production_cycle::stellar_production_cycle;
use crate::game::battle::systems::views::spaceship_view_sync::spaceship_view_sync;
use crate::game::battle::utils::game_time::GameTime;
use crate::game::utils::interop_logger::LoggerRef;

pub(crate) struct EcsContext {
    pub(super) world: World,
    schedule: Schedule,
}

impl EcsContext {
    pub(crate) fn new(logger: LoggerRef) -> EcsContext {
        let mut world = World::new();
        world.insert_resource(GameTime::default());
        world.insert_non_send_resource(logger);

        let mut schedule = Schedule::default();

        schedule.add_systems((stellar_production_cycle, spaceship_move, spaceship_view_sync));

        EcsContext { world, schedule }
    }

    pub(crate) fn update(&mut self, delta_time: f32) {
        let mut time = self.world.resource_mut::<GameTime>();
        time.delta_time = delta_time;

        self.schedule.run(&mut self.world);
    }
}