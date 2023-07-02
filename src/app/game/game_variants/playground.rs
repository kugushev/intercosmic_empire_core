use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::battle::active_battle::ActiveBattle;
use crate::app::game::core::battle_settings::BattleSettings;
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


#[derive(Default)]
pub struct Playground {
    pub battle_settings: BattleSettings,
    pub active_battle: Option<ActiveBattle>,
}

impl Playground {
    pub(crate) fn update(&mut self, delta: DeltaTime) {
        if let Some(battle) = &mut self.active_battle {
            battle.update(delta);
        }
    }
}

