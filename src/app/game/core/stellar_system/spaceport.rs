use std::fmt::{Display, Formatter};
use derive_getters::Getters;
use interoptopus::ffi_type;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;

#[ffi_type]
#[repr(C)]
#[derive(Clone, Getters)]
pub struct Spaceport {
    orbit_radius: f32,
    surface_radius: f32,
}

impl Spaceport {
    pub fn new(size: PlanetSize) -> Self {
        let ratio = size.get_ratio();
        Self {
            orbit_radius: 0.1 * ratio,
            surface_radius: 0.031 * ratio,
        }
    }

    pub fn warpgate() -> Self {
        Self::new(PlanetSize::Mercury)
    }
}

impl Display for Spaceport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Orbit {}, Surface {}", self.orbit_radius, self.surface_radius)
    }
}
