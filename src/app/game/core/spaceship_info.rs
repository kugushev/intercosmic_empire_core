pub mod spaceship_parameters;

use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum SpaceshipMark {
    Viper = 0
}
