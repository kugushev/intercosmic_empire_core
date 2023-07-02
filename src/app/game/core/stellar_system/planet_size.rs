use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum PlanetSize
{
    Mercury,
    Mars,
    Earth,
    Uranus,
    Saturn,
    Jupiter,
}

impl PlanetSize {
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