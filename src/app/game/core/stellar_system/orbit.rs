use interoptopus::ffi_type;
use glam::Vec3;
use crate::app::game::core::stellar_system::StellarSystemInfo;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct Orbit {
    pub radius: f32,
    pub alpha_rotation: f32,
    pub beta_rotation: f32,
    pub start_day: i32,
}

pub const MOCK_START_DAY: i32 = -1;

impl Orbit {
    pub fn get_position(&self, _stellar_system: &StellarSystemInfo) -> Vec3 {
        if self.start_day == MOCK_START_DAY {
            return Vec3::new(self.radius, self.alpha_rotation, self.beta_rotation);
        }

        todo!("generate position based on orbit")
    }
}

#[cfg(test)]
mod tests {
    use crate::app::game::core::stellar_system::orbit::{MOCK_START_DAY, Orbit};

    impl Orbit {
        pub fn new_mock(x: f32, y: f32, z: f32) -> Self {
            Self {
                radius: x,
                alpha_rotation: y,
                beta_rotation: z,
                start_day: MOCK_START_DAY,
            }
        }
    }
}