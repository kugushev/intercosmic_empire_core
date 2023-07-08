use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::battle::entities::productive::Productive;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::orbit::Orbit;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::spaceport::Spaceport;
use crate::app::game::core::stellar_system::StellarSystemParameters;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::random::Random;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct WarpGate {
    pub position: Vec3,
    pub faction: Faction,
    pub production: Production,
    pub current_product: f32,
    pub spaceport: Spaceport,
}

impl WarpGate {
    pub(crate) fn update(&mut self, delta: DeltaTime) {
        self.increment_production(delta);
    }
}

impl WarpGate {
    pub fn new(position: Vec3, faction: Faction) -> Self {
        Self {
            position,
            faction,
            production: Production::new(PlanetSize::Jupiter),
            current_product: 0.0,
            spaceport: Spaceport::new(PlanetSize::Mercury),
        }
    }

    pub fn generate_position(random: &mut Random, stellar_system_parameters: &StellarSystemParameters) -> Vec3 {
        let orbit = Orbit::generate_for_warpgate(random, stellar_system_parameters);
        orbit.get_position(stellar_system_parameters.center, 0)
    }
}

impl Productive for WarpGate {
    fn get_production(&self) -> &Production { &self.production }
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}