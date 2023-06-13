use glam::Vec3;
use crate::game::core::models::faction::Faction;
use interoptopus::{ffi_surrogates, ffi_type};
use crate::ffi_ext::{vec3};
use bevy_ecs::prelude::Resource;
use crate::ffi_models::FFIQuat;
use crate::game::core::models::spaceships::spaceship_mark::SpaceshipMark;

#[derive(Resource, Default)]
pub struct BattleViewsState {
    pub spaceships: Vec<SpaceshipViewModel>
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct SpaceshipViewModel {
    pub position: Vec3,
    pub rotation: FFIQuat,
    pub scale: f32,
    pub faction: Faction,
    pub mark: SpaceshipMark,
}