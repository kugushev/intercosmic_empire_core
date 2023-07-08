pub mod entities;
mod view_model;

use interoptopus::ffi_function;
use crate::app::AppContext;

use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::battle::view_model::BattleViewModel;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::StellarSystemInfo;
use crate::app::game::GameContext;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::struct_vec::StructVec5;
use crate::ffi::utils::{FFIOutcome, FFIResult};

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_update(context: &mut AppContext, delta_time: f32) -> FFIOutcome {
    let guard = &mut context.guard;
    let game = &mut context.game;
    let delta = DeltaTime::new(delta_time);

    let result = guard.wrap(|| {
        let battle = Battle::current_mut(game);
        match battle {
            Some(b) => {
                b.update(delta);
                Ok(())
            }
            None => { Err("Battle not found".to_string()) }
        }
    });
    result.outcome
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_get_vm(context: &mut AppContext) -> FFIResult<BattleViewModel> {
    let guard = &mut context.guard;
    let game = &mut context.game;
    guard.wrap(|| {
        BattleViewModel::current(game)
    })
}

pub struct Battle {
    stellar_system: StellarSystem,
}

impl Battle {
    pub fn new(settings: BattleSettings, stellar_system_info: StellarSystemInfo, stellar_system_faction: Faction, warpgates: StructVec5<WarpGate>) -> Self {
        Self {
            stellar_system: StellarSystem::new(stellar_system_info, stellar_system_faction, warpgates, settings.day_of_year)
        }
    }

    fn current_ref(game_context: &GameContext) -> Option<&Self> {
        let variant = game_context.variant.as_ref()?;
        variant.get_current_battle_ref()
    }

    fn current_mut(game_context: &mut GameContext) -> Option<&mut Self> {
        let variant = game_context.variant.as_mut()?;
        variant.get_current_battle_mut()
    }

    pub fn update(&mut self, delta: DeltaTime) {
        self.stellar_system.update(delta);
    }

    pub fn close(&mut self) -> Result<(), String> {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::app::game::battle::Battle;
    use crate::app::game::core::battle_settings::BattleSettings;
    use crate::app::game::core::faction::Faction;
    use crate::app::game::core::stellar_system::StellarSystemInfo;
    use crate::app::utils::delta_time::DeltaTime;
    use crate::app::utils::struct_vec::StructVec5;

    #[test]
    fn production_on_update() {
        let mut battle = Battle::new(
            BattleSettings::default(),
            StellarSystemInfo::default(),
            Faction::Red,
            StructVec5::default());

        assert_current_product(&mut battle, (0.0, 0.0, 0.0));

        battle.update(DeltaTime::new(1.0));
        assert_current_product(&mut battle, (0.3, 1.5, 5.0));

        battle.update(DeltaTime::new(0.5));
        assert_current_product(&mut battle, (0.3 + 0.15, 1.5 + 0.75, 5.0 + 2.5));

        battle.update(DeltaTime::new(2.0));
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


    // todo: max production case

    // todo: abandoned planet case
}
