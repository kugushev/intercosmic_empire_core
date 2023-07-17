use crate::app::game::core::spaceship_info::SpaceshipMark;

pub struct Deck;

impl Deck {
    pub fn peek_left(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }

    pub fn peek_right(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }
    
    pub fn pop_left(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }

    pub fn pop_right(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }
}