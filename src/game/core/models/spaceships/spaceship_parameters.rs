use crate::game::core::models::spaceships::spaceship_mark::SpaceshipMark;

pub struct SpaceshipParameters {
    pub cost: u8,
    pub mass: f32,
    pub max_speed: f32
}

pub static VIPER: SpaceshipParameters = SpaceshipParameters { cost: 15, mass: 50.0, max_speed: 0.1 };


impl SpaceshipParameters {
    pub fn get_parameters(mark: &SpaceshipMark) -> &SpaceshipParameters {
        match mark {
            SpaceshipMark::Viper => &VIPER,
        }
    }
}
