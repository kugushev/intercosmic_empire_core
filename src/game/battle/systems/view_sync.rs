use bevy_ecs::prelude::{Query, ResMut};
use crate::game::battle::components::translation::Translation;
use crate::game::battle::models::battle_view_model::BattleViewModel;

pub(crate) fn view_sync_translation(mut query: Query<&Translation>, mut view_model: ResMut<BattleViewModel>) {
    for translation in &mut query {
        view_model.test_position = translation.position;
    }
}