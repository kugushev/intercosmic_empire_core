use interoptopus::ffi_type;
use crate::app::game::battle::entities::planet::Planet;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::battle::traits::Spawner;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::struct_vec::StructVec8;

#[ffi_type]
#[repr(C)]
pub struct StellarSystem {
    pub info: StellarSystemInfo,
    pub planets: StructVec8<Planet>,
    pub warpgates: StructVec8<WarpGate>,
}

impl StellarSystem {
    pub fn new(info: StellarSystemInfo, current_faction: Faction, warpgates: StructVec8<WarpGate>, day_of_year: u16, flat_mode: bool) -> Self {
        let planets = info.planets.iter_ref().map(|p| {
            Planet::new(p, current_faction, p.orbit.get_position(info.sun.position, day_of_year, flat_mode))
        }).collect();
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

    pub fn find_spawner(&mut self, spawner_id: i32, planets_only: bool) -> Result<&mut dyn Spawner, String> {
        if !planets_only {
            // try find warpgate
            let found = self.warpgates.iter_mut()
                .find(|x| { x.id.0 == spawner_id });

            if let Some(warpgate) = found {
                return Ok(warpgate);
            }
        }

        // try find planet
        let found = self.planets.iter_mut()
            .find(|x| { x.info.id.0 == spawner_id });

        if let Some(planet) = found {
            return Ok(planet);
        }

        Err(format!("Astronomical Body {spawner_id} not found"))
    }
}