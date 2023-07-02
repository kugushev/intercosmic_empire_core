pub trait Productive {
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
}