use interoptopus::ffi_type;
use bevy_ecs::prelude::Resource;
use crate::game::core::models::stellar_system::StellarSystemId;

#[ffi_type]
#[repr(C)]
#[derive(Resource)]
pub struct BattleParameters{
    pub seed: i32,
    pub stellar_system_id: StellarSystemId
}