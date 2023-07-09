use interoptopus::ffi_type;
use crate::app::game::battle::entities::planet::Planet;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::struct_vec::StructVec5;

#[ffi_type]
#[repr(C)]
pub struct StellarSystem {
    pub info: StellarSystemInfo,
    pub planets: StructVec5<Planet>,
    pub warpgates: StructVec5<WarpGate>,
}

impl StellarSystem {
    pub fn new(info: StellarSystemInfo, current_faction: Faction, warpgates: StructVec5<WarpGate>, day_of_year: u16) -> Self {
        let planets = info.planets.map(|p| {
            Planet::new(p, current_faction, p.orbit.get_position(info.sun.position, day_of_year))
        });
        Self { info, planets, warpgates }
    }

    pub fn update(&mut self, delta: DeltaTime, logger: &LoggerRef) {
        for planet in self.planets.iter_mut() {
            planet.update(delta, logger);
        }

        for warpgate in self.warpgates.iter_mut() {
            warpgate.update(delta, logger);
        }
    }
}