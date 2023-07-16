use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use interoptopus::patterns::primitives::FFIBool;
use crate::app::game::battle::traits::productive::Productive;
use crate::app::game::battle::traits::{AstronomicalBody, Belonging, Spawner};
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::planet_info::PlanetInfo;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::spaceport::Spaceport;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::ffi::surrogates::vec3;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct Planet {
    pub info: PlanetInfo,
    pub position: Vec3,
    pub faction: Faction,
    pub current_product: f32,
    pub under_siege: FFIBool,
}

impl Planet {
    pub fn new(info: &PlanetInfo, faction: Faction, position: Vec3) -> Self {
        Self {
            info: info.clone(),
            position,
            faction,
            current_product: 0.0,
            under_siege: FFIBool::FALSE,
        }
    }

    pub(crate) fn update(&mut self, delta: DeltaTime, _logger: &LoggerRef) {
        if self.under_siege == FFIBool::FALSE {
            self.increment(delta);
        }
    }
}

impl Spawner for Planet {}

impl Productive for Planet {
    fn get_id(&self) -> i32 { self.info.id.0 }
    fn get_production(&self) -> &Production { &self.info.production }
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}

impl AstronomicalBody for Planet {
    fn get_position(&self) -> Vec3 { self.position }
    fn get_spaceport(&self) -> Spaceport { self.info.spaceport.clone() }
}

impl Belonging for Planet {
    fn get_belonging(&self) -> Faction { self.faction }

    fn set_belonging(&mut self, faction: Faction) {
        self.faction = faction;
    }
}