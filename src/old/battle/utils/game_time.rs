use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub(crate) struct GameTime {
    pub delta_time: f32,
}

impl Default for GameTime {
    fn default() -> Self {
        GameTime {
            delta_time: 0.0
        }
    }
}