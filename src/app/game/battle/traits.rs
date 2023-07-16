pub mod productive;

use glam::Vec3;
use crate::app::game::battle::traits::productive::Productive;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::spaceport::Spaceport;

pub trait AstronomicalBody {
    fn get_position(&self) -> Vec3;
    fn get_spaceport(&self) -> Spaceport;
}

pub trait Belonging {
    fn get_belonging(&self) -> Faction;
    fn set_belonging(&mut self, faction: Faction);
}

pub trait Spawner: Productive + AstronomicalBody + Belonging {
    fn receive_damage(&mut self, amount: f32, attacker_belonging: Faction){
        let product_overlap = self.decrease_product(amount);
        if product_overlap > 0.0 {
            // captured
            self.set_belonging(attacker_belonging);
            self.increase_product(product_overlap, true);
        }
    }
}
