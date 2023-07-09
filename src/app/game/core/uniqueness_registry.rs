use crate::app::game::battle::entities::warp_gate::WarpgateId;
use crate::app::game::core::stellar_system::planet_info::PlanetId;
use crate::app::game::core::stellar_system::StellarSystemId;

#[derive(Default)]
pub struct UniquenessRegistry {
    stellar_systems_counter: i32,
    planets_and_warpgates_counter: i32
}

impl UniquenessRegistry {
    pub fn next_stellar_system_id(&mut self) -> StellarSystemId {
        let current = self.stellar_systems_counter;
        self.stellar_systems_counter += 1;
        StellarSystemId(current)
    }

    pub fn next_planet_id(&mut self) -> PlanetId {
        let current = self.planets_and_warpgates_counter;
        self.planets_and_warpgates_counter += 1;
        PlanetId(current)
    }

    pub fn next_warpgate_id(&mut self) -> WarpgateId {
        let current = self.planets_and_warpgates_counter;
        self.planets_and_warpgates_counter += 1;
        WarpgateId(current)
    }
}