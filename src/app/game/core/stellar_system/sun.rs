use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::core::stellar_system::StellarSystemParameters;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct Sun {
    pub position: Vec3,
    pub radius: f32,
}

impl Default for Sun {
    fn default() -> Self {
        let default = StellarSystemParameters::default();
        Self {
            position: default.center,
            radius: default.sun_min_radius,
        }
    }
}