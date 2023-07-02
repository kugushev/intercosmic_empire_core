use interoptopus::ffi_type;
use crate::app::game::core::stellar_system::orbit::Orbit;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::spaceport::Spaceport;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct PlanetInfo {
    pub orbit: Orbit,
    pub size: PlanetSize,
    pub production: Production,
    pub spaceport: Spaceport,
}

impl PlanetInfo {
    pub fn new(orbit: Orbit, size: PlanetSize) -> Self {
        Self {
            orbit,
            size,
            production: Production::new(size),
            spaceport: Spaceport::new(size),
        }
    }
}


