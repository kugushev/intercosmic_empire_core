pub mod spaceship_parameters;

use interoptopus::ffi_type;
use crate::app::game::core::spaceship_info::spaceship_parameters::SpaceshipParameters;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum SpaceshipMark {
    Viper = 0
}

impl SpaceshipMark {
    pub fn parameters(self) -> &'static SpaceshipParameters {
        SpaceshipParameters::get(self)
    }
}