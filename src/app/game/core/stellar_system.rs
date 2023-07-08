pub mod sun;
pub mod planet;
pub mod production;
pub mod spaceport;
pub mod planet_size;
pub mod orbit;

use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::core::stellar_system::orbit::Orbit;
use crate::app::game::core::stellar_system::planet::PlanetInfo;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;
use crate::app::game::core::stellar_system::sun::Sun;
use crate::app::game::core::uniqueness_registry::UniquenessRegistry;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::random::Random;
use crate::app::utils::struct_vec::StructVec5;
use crate::ffi::surrogates::vec3;
use crate::trace;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct StellarSystemId(pub i32);

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
    pub fn new(random: &mut Random, parameters: StellarSystemParameters, uniqueness_registry: &mut UniquenessRegistry, logger_ref: LoggerRef) -> Self {
        let id = uniqueness_registry.next_stellar_system_id();
        let sun = Sun::new(random, &parameters);
        let planets = Self::gen_planets(random, &parameters, uniqueness_registry, logger_ref);

        Self { id, sun, parameters, planets }
    }

    fn gen_planets(random: &mut Random, parameters: &StellarSystemParameters, uniqueness_registry: &mut UniquenessRegistry, logger_ref: LoggerRef) -> StructVec5<PlanetInfo> {
        let mut result = StructVec5::default();

        let planets_count = random.range_inclusive(parameters.min_planets..=parameters.max_planets);

        let mut t = 0.0;

        for _ in 0..planets_count {
            t += 1.0 / planets_count as f32;

            let id = uniqueness_registry.next_planet_id();
            let orbit = Orbit::generate_for_planet(random, parameters, t);
            let size = Self::get_planet_size(random, t);
            let planet = PlanetInfo::new(id, orbit, size);

            if let Err(err) = result.add(planet) {
                trace!(logger_ref, format!("Can't add planet on t={t}: {err}"));
                break;
            }
        }

        result
    }

    fn get_planet_size(random: &mut Random, t: f32) -> PlanetSize {
        match t {
            _ if t < 0.33 => PlanetSize::Mercury,
            _ if t < 0.66 => {
                let random_num = random.range_inclusive(PlanetSize::Mars.num()..=PlanetSize::Earth.num());
                PlanetSize::from_num(random_num)
            }
            _ => {
                let random_num = random.range_inclusive(PlanetSize::Uranus.num()..=PlanetSize::Jupiter.num());
                PlanetSize::from_num(random_num)
            }
        }
    }
}

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

impl StellarSystemParameters {
    pub fn get_warp_gates_radius(&self) -> f32 {
        self.system_radius + 0.1
    }
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
    use crate::app::game::core::stellar_system::planet::{PlanetId, PlanetInfo};
    use crate::app::game::core::stellar_system::planet_size::PlanetSize;
    use crate::app::game::core::stellar_system::sun::Sun;
    use crate::app::utils::struct_vec::StructVec5;


    pub fn create_earth() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.0, 0.2, 0.0);
        PlanetInfo::new(PlanetId(1), Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Earth)
    }

    pub fn create_mercury() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.1, 0.0, 0.0);
        PlanetInfo::new(PlanetId(2), Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Mercury)
    }

    pub fn create_jupiter() -> PlanetInfo {
        let centre = Sun::default().position;
        let p = centre + Vec3::new(0.0, 0.0, 0.3);
        PlanetInfo::new(PlanetId(3), Orbit::new_mock(p.x, p.y, p.z), PlanetSize::Jupiter)
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





