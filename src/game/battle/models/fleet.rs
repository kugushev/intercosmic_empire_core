use interoptopus::ffi_type;
use bevy_ecs::prelude::Resource;
use crate::game::battle::battle_context::BattleContext;
use crate::game::battle::factories::spaceship_factory::create_spaceship;
use crate::game::battle::models::route::Route;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::spaceships::spaceship_model::SpaceshipModel;
use crate::game::core::models::spaceships::spaceship_parameters::SpaceshipParameters;
use crate::game::core::models::stellar_system::production::Productive;

#[ffi_type(opaque)]
#[derive(Resource)]
pub struct Fleet {
    pub faction: Faction,
}

impl Fleet {
    pub fn new(faction: Faction) -> Fleet {
        Fleet {
            faction
        }
    }

    fn spawn_spaceship(&self, ctx: &mut BattleContext, spawner: &mut impl Productive, route: Route,
                       model: SpaceshipModel) {

        let parameters = SpaceshipParameters::from(&model);
        if spawner.try_produce(parameters.cost) {
            create_spaceship(ctx, route, self.faction.clone(), model);
        }
    }
}