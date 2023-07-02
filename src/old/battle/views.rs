use glam::Vec3;
use crate::old::core::models::faction::Faction;
use interoptopus::{ffi_surrogates, ffi_type};
use crate::old_ffi_ext::{vec3};
use bevy_ecs::prelude::Resource;
use crate::old_ffi_models::FFIQuat;
use crate::old::core::models::spaceships::spaceship_mark::SpaceshipMark;

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