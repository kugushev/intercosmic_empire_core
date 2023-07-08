use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum PlanetSize
{
    Mercury = 0,
    Mars = 1,
    Earth = 2,
    Uranus = 3,
    Saturn = 4,
    Jupiter = 5,
}

impl PlanetSize {
    pub fn from_num(num: u8) -> Self {
        match num {
            0 => Self::Mercury,
            1 => Self::Mars,
            2 => Self::Earth,
            3 => Self::Uranus,
            4 => Self::Saturn,
            5 => Self::Jupiter,
            v => panic!("Unexpected cast {v}")
        }
    }

    pub fn num(&self) -> u8 {
        match self {
            PlanetSize::Mercury => 0,
            PlanetSize::Mars => 1,
            PlanetSize::Earth => 2,
            PlanetSize::Uranus => 3,
            PlanetSize::Saturn => 4,
            PlanetSize::Jupiter => 5
        }
    }

    pub fn get_ratio(&self) -> f32 {
        match self {
            PlanetSize::Mercury => { 0.45 }
            PlanetSize::Mars => { 0.55 }
            PlanetSize::Earth => { 0.63 }
            PlanetSize::Uranus => { 0.81 }
            PlanetSize::Saturn => { 0.9 }
            PlanetSize::Jupiter => { 1.0 }
        }
    }
}