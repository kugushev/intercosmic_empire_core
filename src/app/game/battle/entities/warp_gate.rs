use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::app::game::battle::entities::productive::Productive;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::game::core::stellar_system::spaceport::Spaceport;
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
    pub fn new(_seed: i32, _faction: Faction) -> Self {
        // todo: use faction to determine range in sphere
        todo!("Create new random warpgate")
    }
}

impl Productive for WarpGate {
    fn get_production(&self) -> &Production { &self.production }
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}