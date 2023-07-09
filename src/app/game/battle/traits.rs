pub mod productive;

use glam::Vec3;
use crate::app::game::battle::traits::productive::Productive;
use crate::app::game::core::stellar_system::spaceport::Spaceport;

pub trait AstronomicalBody {
    fn get_position(&self) -> Vec3;
    fn get_spaceport(&self) -> Spaceport;
}

pub trait Spawner: Productive + AstronomicalBody {}
