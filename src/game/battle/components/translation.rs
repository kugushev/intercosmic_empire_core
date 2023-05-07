use bevy_ecs::prelude::Component;
use glam::Vec3;

#[derive(Default, Component)]
pub(crate) struct Translation {
    pub(crate) position: Vec3
}