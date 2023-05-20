use interoptopus::{ffi_surrogates, ffi_type};
use glam::Vec3;
use crate::ffi_ext::vec3;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::stellar_system::Production;
use crate::game::core::models::stellar_system::production::Productive;
use crate::game::core::models::stellar_system::spaceport::Spaceport;

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

impl Productive for WarpGate {
    fn current_product(&mut self) -> &mut f32 {
        &mut self.current_product
    }
}