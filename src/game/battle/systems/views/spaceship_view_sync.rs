use bevy_ecs::prelude::Query;
use bevy_ecs::system::ResMut;
use crate::ffi_models::FFIQuat;
use crate::game::battle::components::spaceship::Spaceship;
use crate::game::battle::components::translation::Translation;
use crate::game::battle::views::{BattleViewsState, SpaceshipViewModel};

pub(crate) fn spaceship_view_sync(query: Query<(&Spaceship, &Translation)>, mut battle_view_state: ResMut<BattleViewsState>) {
    battle_view_state.spaceships.clear();
    for (spaceship, translation) in &query {
        let view_model = SpaceshipViewModel {
            position: translation.position,
            rotation: FFIQuat::from(translation.rotation),
            scale: translation.scale,
            faction: spaceship.faction.clone(),
            mark: spaceship.mark,
        };


        battle_view_state.spaceships.push(view_model);
    }
}