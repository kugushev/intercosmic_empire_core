use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::battle::active_battle::ActiveBattle;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::{StellarSystemInfo, StellarSystemParameters};
use crate::app::game::core::uniqueness_registry::UniquenessRegistry;
use crate::app::game::game_variants::GameVariant;
use crate::app::utils::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::struct_vec::StructVec5;
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
    let logger = LoggerRef::new(&context.logger);
    if let Some(GameVariant::Playground(p)) = &mut context.game.variant {
        let result = guard.wrap(|| {
            p.start_battle(logger)
        });
        result.outcome
    } else {
        FFIOutcome::Unable
    }
}

#[derive(Default)]
pub struct Playground {
    battle_settings: BattleSettings,
    stellar_system_parameters: StellarSystemParameters,
    active_battle: Option<ActiveBattle>,
    warpgates: StructVec5<WarpGate>,
    uniqueness_registry: UniquenessRegistry,
}

impl Playground {
    pub fn update(&mut self, delta: DeltaTime) {
        if let Some(battle) = &mut self.active_battle {
            battle.update(delta);
        }
    }

    pub fn add_warpgate(&mut self, faction: Faction) -> Result<(), String> {
        self.warpgates.add(
            WarpGate::new(self.battle_settings.seed, faction))
    }

    pub fn start_battle(&mut self, logger_ref: LoggerRef) -> Result<(), String> {
        if let Some(_) = self.active_battle {
            return Err("Battle is already active".to_string());
        }

        let stellar_system_info = StellarSystemInfo::new(
            self.battle_settings.seed,
            self.stellar_system_parameters.clone(),
            &mut self.uniqueness_registry,
            logger_ref
        );

        self.active_battle = Some(ActiveBattle::new(
            self.battle_settings.clone(),
            stellar_system_info,
            Faction::Red,
            self.warpgates.clone(),
        ));

        Ok(())
    }
}

