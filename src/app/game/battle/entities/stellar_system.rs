use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::battle::entities::productive::Productive;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::planet::PlanetInfo;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::utils::DeltaTime;
use crate::app::utils::struct_vec::StructVec5;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[repr(C)]
pub struct StellarSystem {
    pub info: StellarSystemInfo,
    pub planets: StructVec5<Planet>,
}

impl StellarSystem {
    pub fn new(info: StellarSystemInfo, current_faction: Faction) -> Self {
        let planets = info.planets.map(|p| {
            Planet::new(p, current_faction, p.orbit.get_position(&info))
        });
        Self { info, planets }
    }

    pub fn update(&mut self, delta: DeltaTime) {
        for planet in self.planets.iter_mut() {
            planet.update(delta);
        }
    }
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct Planet {
    pub info: PlanetInfo,
    pub position: Vec3,
    pub faction: Faction,
    pub current_product: f32,
    pub under_siege: bool,
}

impl Planet {
    pub fn new(info: &PlanetInfo, faction: Faction, position: Vec3) -> Self {
        Self {
            info: info.clone(),
            position,
            faction,
            current_product: 0.0,
            under_siege: false,
        }
    }

    pub(crate) fn update(&mut self, delta: DeltaTime) {
        self.increment_production(delta);
    }
}

impl Productive for Planet {
    fn get_production(&self) -> &Production { &self.info.production }
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}