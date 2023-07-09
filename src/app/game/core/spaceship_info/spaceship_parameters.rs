use crate::app::game::core::spaceship_info::SpaceshipMark;

pub struct SpaceshipParameters {
    pub cost: u8,
    pub mass: f32,
    pub max_speed: f32
}

pub static VIPER: SpaceshipParameters = SpaceshipParameters { cost: 15, mass: 50.0, max_speed: 0.1 };

impl SpaceshipParameters {
    pub fn get(mark: SpaceshipMark) -> &'static SpaceshipParameters {
        match mark {
            SpaceshipMark::Viper => &VIPER,
        }
    }
}
