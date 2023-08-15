use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::battle::traits::productive::Productive;
use crate::app::game::battle::traits::{AstronomicalBody, Belonging, Spawner};
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::orbit::Orbit;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::spaceport::Spaceport;
use crate::app::game::core::stellar_system::StellarSystemParameters;
use crate::app::game::core::uniqueness_registry::UniquenessRegistry;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::random::Random;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct WarpgateId(pub i32);

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct WarpGate {
    pub id: WarpgateId,
    pub position: Vec3,
    pub faction: Faction,
    pub production: Production,
    pub current_product: f32,
    pub spaceport: Spaceport,
}

impl WarpGate {
    pub(crate) fn update(&mut self, delta: DeltaTime, _logger: &LoggerRef) {
        self.increment(delta);
    }
}

impl WarpGate {
    pub fn new(position: Vec3, faction: Faction, uniqueness_registry: &mut UniquenessRegistry) -> Self {
        let production = Production::warpgate();
        let current_product = production.max_product;
        Self {
            id: uniqueness_registry.next_warpgate_id(),
            position,
            faction,
            production,
            current_product,
            spaceport: Spaceport::warpgate(),
        }
    }

    pub fn generate_position(random: &mut Random, stellar_system_parameters: &StellarSystemParameters, flat_mode: bool) -> Vec3 {
        let orbit = Orbit::generate_for_warpgate(random, stellar_system_parameters);
        orbit.get_position(stellar_system_parameters.center, 0, flat_mode)
    }
}

impl Spawner for WarpGate {}

impl Productive for WarpGate {
    fn get_id(&self) -> i32 { self.id.0 }
    fn get_production(&self) -> &Production { &self.production }
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}

impl AstronomicalBody for WarpGate {
    fn get_position(&self) -> Vec3 { self.position }
    fn get_spaceport(&self) -> Spaceport { self.spaceport.clone() }
}

impl Belonging for WarpGate {
    fn get_belonging(&self) -> Faction { self.faction }

    fn set_belonging(&mut self, faction: Faction) {
        self.faction = faction;
    }
}