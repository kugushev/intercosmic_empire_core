use bevy_ecs::prelude::Resource;
use interoptopus::{ffi_type, ffi_surrogates};
use glam::Vec3;
use crate::ffi_ext::vec3;
use crate::game::core::models::faction::Faction;

#[derive(Resource, Clone)]
pub struct StellarSystem {
    pub id: StellarSystemId,
    pub sun: Sun,
    pub parameters: StellarSystemParameters,
    pub planets: Vec<Planet>,
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
            min_planets: 3,
            max_planets: 6,
        }
    }
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct Sun {
    pub position: Vec3,
    pub radius: f32,
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
#[derive(Clone)]
pub struct Planet {
    pub info: PlanetInfo,
    pub position: Vec3,
    pub faction: Faction,
    pub current_product: f32,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct PlanetId(pub i32);

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct PlanetInfo {
    pub id: PlanetId,
    pub orbit: Orbit,
    pub size: PlanetSize,
    pub production: Production,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Orbit {
    pub radius: f32,
    pub alpha_rotation: f32,
    pub beta_rotation: f32,
    pub start_day: i32,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub enum PlanetSize
{
    Mercury,
    Mars,
    Earth,
    Uranus,
    Saturn,
    Jupiter,
}

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Production {
    pub amount_per_second: f32,
    pub max_product: f32,
}