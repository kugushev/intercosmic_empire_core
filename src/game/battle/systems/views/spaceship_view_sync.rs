use bevy_ecs::prelude::Query;
use bevy_ecs::system::ResMut;
use crate::game::battle::components::spaceship::Spaceship;
use crate::game::battle::components::translation::Translation;
use crate::game::battle::views::{BattleViewsState, SpaceshipView};

pub(crate) fn spaceship_view_sync(query: Query<(&Spaceship, &Translation)>, mut battle_view_state: ResMut<BattleViewsState>) {
    battle_view_state.spaceships.clear();
    for (spaceship, translation) in &query {
        battle_view_state.spaceships.push(SpaceshipView {
            position: translation.position,
            rotation: translation.rotation,
            scale: translation.scale,
            faction: spaceship.faction.clone(),
        })
    }
}