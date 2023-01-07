use bevy_ecs::prelude::{Query, Res};
use crate::game::battle::components::translation::Translation;
use crate::game::battle::utils::game_time::GameTime;

pub(crate) fn test_system(mut query: Query<&mut Translation>, time: Res<GameTime>) {

    for mut translation in &mut query{
        translation.position += time.delta_time;
    }
}