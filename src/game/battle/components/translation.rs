use bevy_ecs::prelude::Component;
use glam::{Quat, Vec3};

#[derive(Component)]
pub struct Translation {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
}