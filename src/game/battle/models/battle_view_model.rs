use bevy_ecs::prelude::Resource;
use glam::Vec3;
use interoptopus::{ffi_type, ffi_surrogates};
use crate::ffi_ext::vec3;

#[ffi_type]
#[ffi_surrogates(test_position = "vec3")]
#[repr(C)]
#[derive(Default, Resource)]
pub struct BattleViewModel {
    pub test_position: Vec3,
}