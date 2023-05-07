use crate::game::core::models::spaceships::spaceship_model::SpaceshipModel;

pub struct SpaceshipParameters {
    pub cost: u8,
}

// todo: push all params to registry and use via ref
impl SpaceshipParameters {
    pub fn from(model: &SpaceshipModel) -> SpaceshipParameters {
        match model {
            SpaceshipModel::Viper => SpaceshipParameters { cost: 15 },
        }
    }
}
