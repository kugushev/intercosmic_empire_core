use interoptopus::ffi_type;
use glam::{EulerRot, Quat, Vec3};
use lerp::Lerp;
use crate::app::game::core::stellar_system::StellarSystemParameters;
use crate::app::utils::random::Random;

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
pub const ELLIPSE_BETA_MULTIPLIER: f32 = 2.0;
pub const ALL_ORBITS_ANGLE: f32 = 45.0;

impl Orbit {
    pub(crate) fn generate_for_planet(random: &mut Random, parameters: &StellarSystemParameters, t: f32) -> Self {
        let radius = 0.0.lerp(parameters.system_radius, t) + parameters.min_distance_to_sun;

        let alpha_variation = (1.0 - t) * 180.0;
        let alpha_rotation = random.range(-alpha_variation..alpha_variation);

        // should be smaller to avoid touching player belly
        let beta_variation = alpha_variation / ELLIPSE_BETA_MULTIPLIER;
        let beta_rotation = random.range(-beta_variation..beta_variation);

        let start_day = Self::generate_start_day(random);

        Self { radius, alpha_rotation, beta_rotation, start_day }
    }

    pub(crate) fn generate_for_warpgate(random: &mut Random, parameters: &StellarSystemParameters) -> Self {
        let start_day = Self::generate_start_day(random);
        Self {
            radius: parameters.get_warp_gates_radius(),
            alpha_rotation: 0.0,
            beta_rotation: 0.0,
            start_day,
        }
    }

    fn generate_start_day(random: &mut Random) -> i32 { random.range_inclusive(0..=360) }

    pub fn get_position(&self, sun_position: Vec3, day_of_year: u16, flat_mode: bool) -> Vec3 {
        if self.start_day == MOCK_START_DAY {
            return Vec3::new(self.radius, self.alpha_rotation, self.beta_rotation);
        }

        let angle = (day_of_year as f32 + self.start_day as f32).to_radians();
        let small_radius = self.radius / ELLIPSE_BETA_MULTIPLIER;

        let x = sun_position.x + self.radius * angle.cos();
        let y = sun_position.y + small_radius * angle.sin();
        let z = sun_position.z;

        let flat_position = Vec3::new(x, y, z);

        if flat_mode {
            return flat_position;
        }

        let rotation = Quat::from_euler(EulerRot::default(),
                                        self.alpha_rotation + ALL_ORBITS_ANGLE,
                                        self.beta_rotation
                                        , 0.0);

        rotation * (flat_position - sun_position) + sun_position
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