use interoptopus::{ffi_type, ffi_surrogates};
use glam::Vec3;
use crate::ffi_ext::vec3;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::stellar_system::Production;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct WarpGate {
    pub position: Vec3,
    pub faction: Faction,
    pub production: Production,
    pub current_product: f32,
}