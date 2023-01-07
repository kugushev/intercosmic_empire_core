use interoptopus::ffi_type;
use bevy_ecs::prelude::Resource;

#[ffi_type]
#[repr(C)]
#[derive(Resource)]
pub struct BattleParameters{
    pub seed: i32
}