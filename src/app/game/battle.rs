mod battle_view_model;
pub mod entities;
pub mod traits;
pub mod services;
pub mod ai_agents;
pub mod constants;

use interoptopus::ffi_function;
use interoptopus::patterns::primitives::FFIBool;
use crate::app::{AppContext, AppSettings};
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::battle::battle_view_model::BattleViewModel;
use crate::app::game::battle::constants::{Constants, CONSTANTS};
use crate::app::game::battle::entities::fleet_set::FleetSet;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::game::GameContext;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::struct_vec::StructVec8;
use crate::ffi::utils::{FFIOutcome, FFIResult};

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_is_active(context: &mut AppContext) -> FFIResult<FFIBool> {
    let guard = &mut context.guard;
    let game = &mut context.game;
    guard.wrap(|| {
        let battle = Battle::current_ref(game);
        Ok(battle.is_some().into())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_update(context: &mut AppContext, delta_time: f32) -> FFIOutcome {
    let logger = LoggerRef::new(&context.logger);
    let result = current_battle_exec(context, |b| {
        let delta = DeltaTime::new(delta_time);
        b.update(delta, &logger);
        Ok(())
    });
    result.outcome
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_get_vm(context: &mut AppContext) -> FFIResult<BattleViewModel> {
    let game = &mut context.game;
    let guard = &mut context.guard;

    guard.wrap(|| {
        let battle = current_battle_mut(game)?;
        BattleViewModel::new(battle)
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_get_settings(context: &mut AppContext) -> FFIResult<BattleSettings> {
    let game = &mut context.game;
    let guard = &mut context.guard;

    guard.wrap(|| {
        let battle = current_battle_ref(game)?;
        Ok(battle.settings.clone())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_get_constants() -> Constants {
    CONSTANTS.clone()
}

pub struct Battle {
    stellar_system: StellarSystem,
    fleets: FleetSet,
    settings: BattleSettings
}

impl Battle {
    pub fn new(settings: BattleSettings, stellar_system_info: StellarSystemInfo, stellar_system_faction: Faction, warpgates: StructVec8<WarpGate>, logger: &LoggerRef, app_settings: &AppSettings) -> Self {
        let stellar_system = StellarSystem::new(stellar_system_info,
                                                stellar_system_faction,
                                                warpgates,
                                                settings.day_of_year,
                                                app_settings.flat_mode);
        let fleets = FleetSet::new(&settings, logger);
        Self { stellar_system, fleets, settings }
    }

    fn current_ref(game_context: &GameContext) -> Option<&Self> {
        let variant = game_context.variant.as_ref()?;
        variant.get_current_battle_ref()
    }

    fn current_mut(game_context: &mut GameContext) -> Option<&mut Self> {
        let variant = game_context.variant.as_mut()?;
        variant.get_current_battle_mut()
    }

    pub fn update(&mut self, delta: DeltaTime, logger: &LoggerRef) {
        self.stellar_system.update(delta, logger);
        self.fleets.update(&mut self.stellar_system, delta, logger);
    }

    pub fn close(&mut self) -> Result<(), String> {
        Ok(())
    }
}

pub fn current_battle_exec<T>(context: &mut AppContext, action: impl FnOnce(&mut Battle) -> Result<T, String>) -> FFIResult<T> {
    let guard = &mut context.guard;
    let game = &mut context.game;
    guard.wrap(|| {
        let battle = Battle::current_mut(game);
        match battle {
            Some(b) => {
                action(b)
            }
            None => { Err("Current battle not found".to_string()) }
        }
    })
}

pub fn current_battle_ref(game: &GameContext) -> Result<&Battle, String> {
    let battle = Battle::current_ref(game);
    match battle {
        Some(b) => {
            Ok(b)
        }
        None => { Err("Current battle not found".to_string()) }
    }
}

pub fn current_battle_mut(game: &mut GameContext) -> Result<&mut Battle, String> {
    let battle = Battle::current_mut(game);
    match battle {
        Some(b) => {
            Ok(b)
        }
        None => { Err("Current battle not found".to_string()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::AppSettings;
    use crate::app::game::battle::Battle;
    use crate::app::game::core::battle_settings::BattleSettings;
    use crate::app::game::core::faction::Faction;
    use crate::app::game::core::stellar_system::StellarSystemInfo;
    use crate::app::utils::delta_time::DeltaTime;
    use crate::app::utils::interop_logger::LoggerRef;
    use crate::app::utils::struct_vec::StructVec8;

    #[test]
    fn production_on_update() {
        let mut battle = Battle::new(
            BattleSettings::default(),
            StellarSystemInfo::default(),
            Faction::Enemy,
            StructVec8::default(),
            &LoggerRef::default(),
            &AppSettings::default()
        );

        assert_current_product(&mut battle, (0.0, 0.0, 0.0));

        battle.update(DeltaTime::new(1.0), &LoggerRef::default());
        assert_current_product(&mut battle, (0.3, 1.5, 5.0));

        battle.update(DeltaTime::new(0.5), &LoggerRef::default());
        assert_current_product(&mut battle, (0.3 + 0.15, 1.5 + 0.75, 5.0 + 2.5));

        battle.update(DeltaTime::new(2.0), &LoggerRef::default());
        assert_current_product(&mut battle, (0.3 + 0.15 + 0.6, 1.5 + 0.75 + 3.0, 5.0 + 2.5 + 10.0));

        fn assert_current_product(battle: &mut Battle, expected: (f32, f32, f32)) {
            let mut iter = battle.stellar_system.planets.iter_mut();
            let mercury = iter.next().unwrap().current_product;
            let earth = iter.next().unwrap().current_product;
            let jupiter = iter.next().unwrap().current_product;

            assert_eq!(mercury, expected.0);
            assert_eq!(earth, expected.1);
            assert_eq!(jupiter, expected.2);
        }
    }
}
