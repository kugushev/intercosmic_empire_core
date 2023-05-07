use crate::game::battle::battle_context::BattleContext;
use crate::game::battle::components::spaceship::{Spaceship, SpaceshipBundle};
use crate::game::battle::components::translation::Translation;
use crate::game::battle::models::route::Route;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::spaceships::spaceship_model::SpaceshipModel;

pub fn create_spaceship(
    ctx: &mut BattleContext,
    route: Route,
    faction: Faction,
    model: SpaceshipModel,
) {
    ctx.ecs.world.spawn(SpaceshipBundle {
        spaceship: Spaceship {
            route,
            faction,
            model,
        },
        translation: Translation::default(),
    });
}
