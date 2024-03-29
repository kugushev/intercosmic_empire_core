use interoptopus::ffi_function;
use interoptopus::patterns::primitives::FFIBool;
use crate::app::AppContext;
use crate::app::game::battle::current_battle_exec;
use crate::app::game::core::stellar_system::production::Production;
use crate::app::utils::delta_time::DeltaTime;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_productive_can_produce(
    context: &mut AppContext,
    spawner_id: i32,
    cost: u8,
) -> FFIResult<FFIBool> {
    current_battle_exec(context, |b| {
        let stellar_system = &mut b.stellar_system;
        let spawner = stellar_system.find_spawner(spawner_id, false)?;
        let can = spawner.can_produce(cost);
        Ok(can.into())
    })
}

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

    fn increment(&mut self, delta: DeltaTime) {
        let amount_per_second = self.get_production().amount_per_second;
        self.increase_product(amount_per_second * delta.seconds(), false)
    }

    fn increase_product(&mut self, amount: f32, can_overlap: bool) {
        let max_product = self.get_production().max_product;
        let current_ref = self.current_product();
        let current = *current_ref;
        let max = if current <= max_product { max_product } else { current };
        let new_product = current + amount;
        if can_overlap {
            *current_ref = new_product;
        } else {
            *current_ref = new_product.min(max);
        }
    }

    fn decrease_product(&mut self, amount: f32) -> f32 {
        let current = self.current_product();
        *current -= amount;
        if *current < 0.0 {
            let overlap = -*current;
            *current = 0.0;
            overlap
        } else { 0.0 }
    }
}