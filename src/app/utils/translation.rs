use glam::{Quat, Vec3};
use interoptopus::{ffi_surrogates, ffi_type};
use crate::ffi::surrogates::vec3;
use crate::ffi::utils::FFIQuat;

pub struct Translation {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: f32,
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct FFITranslation {
    pub position: Vec3,
    pub rotation: FFIQuat,
    pub scale: f32,
}

impl From<&Translation> for FFITranslation {
    fn from(value: &Translation) -> Self {
        Self {
            position: value.position,
            rotation: FFIQuat::from(value.rotation),
            scale: value.scale,
        }
    }
}

