pub mod sun;
pub mod planet;
pub mod production;
pub mod spaceport;
pub mod planet_size;
pub mod orbit;

use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::core::stellar_system::planet::PlanetInfo;
use crate::app::game::core::stellar_system::sun::Sun;
use crate::app::utils::struct_vec::StructVec5;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct StellarSystemInfo {
    pub id: StellarSystemId,
    pub sun: Sun,
    pub parameters: StellarSystemParameters,
    pub planets: StructVec5<PlanetInfo>,
}

impl StellarSystemInfo {
    pub fn new(_seed: i32, _parameters: StellarSystemParameters) -> Self {
        todo!("Create random stellar system, based on StellarSystemParameters")
    }
}

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct StellarSystemId(pub i32);

#[ffi_type]
#[ffi_surrogates(center = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct StellarSystemParameters {
    pub system_radius: f32,
    pub min_distance_to_sun: f32,
    pub center: Vec3,
    pub sun_min_radius: f32,
    pub sun_max_radius: f32,
    pub min_planets: i32,
    pub max_planets: i32,
}

impl Default for StellarSystemParameters {
    fn default() -> Self {
        StellarSystemParameters {
            system_radius: 0.8,
            min_distance_to_sun: 0.1,
            center: Vec3::new(0.0, 1.5, 0.5),
            sun_min_radius: 0.05,
            sun_max_radius: 0.15,
            min_planets: 2,
            max_planets: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;
    use crate::app::game::core::stellar_system::{StellarSystemId, StellarSystemInfo, StellarSystemParameters};
    use crate::app::game::core::stellar_system::orbit::Orbit;
    use crate::app::game::core::stellar_system::planet::PlanetInfo;
    use crate::app::game::core::stellar_system::planet_size::PlanetSize;
    use crate::app::game::core::stellar_system::sun::Sun;
    use crate::app::utils::struct_vec::StructVec5;


    pub fn create_earth() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.0, 0.2, 0.0);
        PlanetInfo::new(Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Earth)
    }

    pub fn create_mercury() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.1, 0.0, 0.0);
        PlanetInfo::new(Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Mercury)
    }

    pub fn create_jupiter() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.0, 0.0, 0.3);
        PlanetInfo::new(Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Jupiter)
    }

    impl Default for StellarSystemInfo {
        fn default() -> Self {
            Self {
                id: StellarSystemId(0),
                sun: Sun::default(),
                parameters: StellarSystemParameters::default(),
                planets: StructVec5::new3(create_mercury(), create_earth(), create_jupiter()),
            }
        }
    }
}





