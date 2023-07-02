use std::thread::sleep;
use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::battle::active_battle::ActiveBattle;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::{StellarSystemInfo, StellarSystemParameters};
use crate::app::game::game_variants::GameVariant;
use crate::app::utils::DeltaTime;
use crate::ffi::utils::{FFIOutcome, FFIResult};

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_get_battle_settings(context: &mut AppContext) -> FFIResult<BattleSettings> {
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        return FFIResult::ok(p.battle_settings.clone());
    }

    FFIResult::unable()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_set_battle_settings(context: &mut AppContext, settings: BattleSettings) -> FFIOutcome {
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        p.battle_settings = settings;
        FFIOutcome::Ok
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_get_stellar_system_parameters(context: &mut AppContext) -> FFIResult<StellarSystemParameters> {
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        return FFIResult::ok(p.stellar_system_parameters.clone());
    }

    FFIResult::unable()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_set_stellar_system_parameters(context: &mut AppContext, parameters: StellarSystemParameters) -> FFIOutcome {
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        p.stellar_system_parameters = parameters;
        FFIOutcome::Ok
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_add_warpgate(context: &mut AppContext, faction: Faction) -> FFIOutcome {
    let guard = &mut context.guard;
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        let result = guard.wrap(|| {
            p.add_warpgate(faction)
        });
        result.outcome
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_game_playground_start(context: &mut AppContext) -> FFIOutcome {
    let guard = &mut context.guard;
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        let result = guard.wrap(|| {
            p.start_battle()
        });
        result.outcome
    } else {
        FFIOutcome::Unable
    }
}

#[derive(Default)]
pub struct Playground {
    pub battle_settings: BattleSettings,
    pub stellar_system_parameters: StellarSystemParameters,
    pub active_battle: Option<ActiveBattle>,
    pub warpgates: Vec<WarpGate>,
}

const MAX_WARPGATES: usize = 5;

impl Playground {
    pub fn update(&mut self, delta: DeltaTime) {
        if let Some(battle) = &mut self.active_battle {
            battle.update(delta);
        }
    }

    pub fn add_warpgate(&mut self, faction: Faction) -> Result<(), String> {
        if self.warpgates.len() > MAX_WARPGATES {
            return Err("Too much warpgates".to_string());
        }
        self.warpgates.push(WarpGate::new(self.battle_settings.seed, faction));
        Ok(())
    }

    pub fn start_battle(&mut self) -> Result<(), String> {
        if let Some(_) = self.active_battle {
            return Err("Battle is already active".to_string());
        }

        self.active_battle = Some(ActiveBattle::new(
            StellarSystemInfo::new(self.battle_settings.seed,
                                   self.stellar_system_parameters.clone()),
            Faction::Red,
        ));

        Ok(())
    }
}

