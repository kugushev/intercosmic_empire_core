use interoptopus::ffi_type;
use crate::app::game::core::stellar_system::planet_size::PlanetSize;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Production {
    pub amount_per_second: f32,
    pub max_product: f32,
}

impl Production {
    pub fn new(size: PlanetSize) -> Self {
        Self {
            amount_per_second: Self::get_amount_per_second(size),
            max_product: Self::get_max_product(size),
        }
    }

    pub fn warpgate() -> Self {
        // default value, should be the summ of the original planets power
        Self {
            amount_per_second: 2.5,
            max_product: 50.0
        }
    }

    fn get_amount_per_second(size: PlanetSize) -> f32 {
        match size {
            PlanetSize::Mercury => { 0.3 }
            PlanetSize::Mars => { 1.0 }
            PlanetSize::Earth => { 1.5 }
            PlanetSize::Uranus => { 3.0 }
            PlanetSize::Saturn => { 4.0 }
            PlanetSize::Jupiter => { 5.0 }
        }
    }

    fn get_max_product(size: PlanetSize) -> f32 {
        match size {
            PlanetSize::Mercury => { 20.0 }
            PlanetSize::Mars => { 40.0 }
            PlanetSize::Earth => { 50.0 }
            PlanetSize::Uranus => { 80.0 }
            PlanetSize::Saturn => { 90.0 }
            PlanetSize::Jupiter => { 100.0 }
        }
    }
}