use crate::app::game::core::stellar_system::StellarSystemId;

#[derive(Default)]
pub struct UniquenessRegistry {
    stellar_systems_counter: u16
}

impl UniquenessRegistry {
    pub fn next_stellar_system_id(&mut self) -> StellarSystemId {
        let current = self.stellar_systems_counter;
        self.stellar_systems_counter += 1;
        StellarSystemId(current)
    }
}