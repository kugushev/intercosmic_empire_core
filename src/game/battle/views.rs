use glam::{Quat, Vec3};
use crate::game::core::models::faction::Faction;
use interoptopus::{ffi_surrogates, ffi_type};
use crate::ffi_ext::{vec3, quat};
use bevy_ecs::prelude::Resource;

#[derive(Default, Resource)]
pub struct BattleViewsState {
    pub spaceships: Vec<SpaceshipView>,
}


#[ffi_type]
#[ffi_surrogates(position = "vec3", rotation = "quat")]
#[repr(C)]
pub struct SpaceshipView {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
    pub faction: Faction,
}