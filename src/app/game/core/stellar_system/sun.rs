use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use rand::Rng;
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

impl Sun {
    pub fn new(rng: &mut impl Rng, p: &StellarSystemParameters) -> Self {
        let radius = rng.gen_range(p.sun_min_radius..p.sun_max_radius);
        Self {
            position: p.center,
            radius
        }
    }
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