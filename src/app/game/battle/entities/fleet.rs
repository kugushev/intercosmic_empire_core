pub mod fleet_view_model;
pub mod fleet_spawn;

use std::iter::Zip;
use std::slice::IterMut;
use crate::app::AppContext;
use crate::app::game::battle::{current_battle_exec, current_battle_ref};
use crate::app::game::battle::entities::fleet::fleet_spawn::SpawnInfo;
use crate::app::game::battle::entities::route::{Route, RouteBuilders, RouteBuildersSource};
use crate::app::game::battle::entities::spaceship::{EMPTY_SPACESHIP_VM, Spaceship, SpaceshipViewModel};
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::app::game::core::spaceship_info::spaceship_parameters::SpaceshipParameters;
use crate::app::game::GameContext;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::ffi::utils::FFIResult;


pub struct Fleet {
    faction: Faction,
    route_builders: RouteBuilders,
    spaceships: Vec<Option<Spaceship>>,
    spaceships_counter: u64,
    spaceships_vm: Vec<SpaceshipViewModel>,
}

impl Fleet {
    pub fn new(faction: Faction) -> Self {
        Self {
            faction,
            route_builders: RouteBuilders::default(),
            spaceships: Vec::new(),
            spaceships_counter: EMPTY_SPACESHIP_VM,
            spaceships_vm: Vec::new(),
        }
    }

    pub fn update(&mut self, stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) {
        for (slot, vm) in self.zip_spaceships() {
            if let Some(spaceship) = slot {
                spaceship.update(stellar_system, delta, logger);

                if spaceship.disposed() {
                    *slot = None;
                    *vm = SpaceshipViewModel::empty()
                } else {
                    *vm = spaceship.get_vm()
                }
            }
        }
    }

    pub fn spawn_prepare(&mut self, spawner_id: i32, stellar_system: &mut StellarSystem, builder_source: RouteBuildersSource, spaceship_mark: SpaceshipMark, faction: Faction) -> Result<SpawnInfo, String> {
        let builders = &mut self.route_builders;
        let spawner = stellar_system.find_spawner(spawner_id, false)?;

        if spawner.get_belonging() != faction {
            return Err(format!("Unexpected faction {faction:?}, spawner faction is {:?}", spawner.get_belonging()));
        }

        let builder_id = builders.new_builders(builder_source,
                                               spawner.get_position(),
                                               spawner.get_spaceport())?;
        Ok(SpawnInfo {
            faction: self.faction,
            spawner_id,
            builder_source,
            builder_id,
            spaceship_mark,
        })
    }

    pub fn spawn_finish(&mut self, info: SpawnInfo, finish_id: i32, stellar_system: &mut StellarSystem) -> Result<(), String> {
        let builders = &mut self.route_builders;
        let finish = stellar_system.find_spawner(finish_id, true)?;
        let route = builders.finish(info.builder_source, info.builder_id,
                                    finish.get_position(),
                                    finish.get_spaceport(),
                                    finish_id)?;

        let spawner = stellar_system.find_spawner(info.spawner_id, false)?;
        let spaceship_parameters = SpaceshipParameters::get(info.spaceship_mark);
        if !spawner.try_produce(spaceship_parameters.cost) {
            return Err(format!("Can't produce spaceship {}, current {}", spaceship_parameters.cost, spawner.current_product()));
        }

        self.spawn_spaceship(info.spaceship_mark, route)
    }

    fn spawn_spaceship(&mut self, mark: SpaceshipMark, route: Route) -> Result<(), String> {
        self.spaceships_counter += 1;
        if self.spaceships_counter == EMPTY_SPACESHIP_VM {
            self.spaceships_counter += 1;
        }

        let spaceship = Spaceship::new(self.spaceships_counter, mark, route, self.faction);

        let found_slot = self.find_slot();

        if let Some((slot, slot_vm)) = found_slot {
            *slot_vm = spaceship.get_vm();
            *slot = Some(spaceship);
        } else {
            self.spaceships_vm.push(spaceship.get_vm());
            self.spaceships.push(Some(spaceship));
        }

        Ok(())
    }

    fn find_slot(&mut self) -> Option<(&mut Option<Spaceship>, &mut SpaceshipViewModel)> {
        let mut found_slot: Option<(&mut Option<Spaceship>, &mut SpaceshipViewModel)> = None;
        for (slot, slot_vm) in self.zip_spaceships() {
            if let Some(spaceship_in_slot) = &slot {
                if !spaceship_in_slot.disposed() {
                    continue;
                }
            }
            found_slot = Some((slot, slot_vm));
            break;
        };
        found_slot
    }

    fn zip_spaceships(&mut self) -> Zip<IterMut<Option<Spaceship>>, IterMut<SpaceshipViewModel>> {
        self.spaceships.iter_mut().zip(self.spaceships_vm.iter_mut())
    }
}


pub fn fleet_exec<T>(context: &mut AppContext, faction: Faction, action: impl FnOnce(&mut Fleet) -> Result<T, String>) -> FFIResult<T> {
    current_battle_exec(context, |b| {
        if let Some(fleet) = b.fleets.get_mut(&faction) {
            action(fleet)
        } else {
            Err(format!("Fleet not found {faction:?}"))
        }
    })
}

pub fn fleet_ref(game: &GameContext, faction: Faction) -> Result<&Fleet, String> {
    let battle = current_battle_ref(game)?;
    if let Some(fleet) = battle.fleets.get(&faction) {
        Ok(fleet)
    } else {
        Err(format!("Fleet not found {faction:?}"))
    }
}
