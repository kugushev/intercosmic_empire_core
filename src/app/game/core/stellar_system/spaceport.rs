use interoptopus::ffi_type;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Spaceport {
    pub orbit_radius: f32,
    pub surface_radius: f32
}

impl Spaceport {
    pub fn new(size: PlanetSize) -> Self {
        let ratio = size.get_ratio();
        Self {
            orbit_radius: 0.1 * ratio,
            surface_radius: 0.031 * ratio
        }
    }
}
