use crate::app::game::core::stellar_system::production::Production;
use crate::app::utils::delta_time::DeltaTime;

pub trait Productive {
    fn get_id(&self) -> i32;

    fn get_production(&self) -> &Production;

    fn current_product(&mut self) -> &mut f32;

    fn try_produce(&mut self, cost: u8) -> bool {
        let current = self.current_product();
        let cost_f32 = cost as f32;
        if *current >= cost_f32 {
            *current -= cost_f32;
            return true;
        }
        false
    }

    fn can_produce(&mut self, cost: u8) -> bool {
        let current = self.current_product();
        let cost_f32 = cost as f32;
        *current >= cost_f32
    }

    fn increment_production(&mut self, delta: DeltaTime) {
        let production = self.get_production().clone();
        let current = self.current_product();
        let new_product = *current + production.amount_per_second * delta.seconds();
        *current = new_product.min(production.max_product);
    }
}