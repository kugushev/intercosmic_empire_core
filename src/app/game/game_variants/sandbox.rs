use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::battle::Battle;
use crate::app::game::battle::entities::warp_gate::WarpGate;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::stellar_system::{StellarSystemInfo, StellarSystemParameters};
use crate::app::game::core::uniqueness_registry::UniquenessRegistry;
use crate::app::game::game_variants::GameVariant;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::random::Random;
use crate::app::utils::struct_vec::StructVec5;
use crate::ffi::utils::{FFIOutcome, FFIResult};

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_close(context: &mut AppContext) -> FFIOutcome {
    let guard = &mut context.guard;
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        let result = guard.wrap(|| p.close());

        // Don't forget changing the current variant
        if result.outcome == FFIOutcome::Ok {
            context.game.variant = None;
        }

        result.outcome
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_get_battle_settings(context: &mut AppContext) -> FFIResult<BattleSettings> {
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        return FFIResult::ok(p.battle_settings.clone());
    }

    FFIResult::unable()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_set_battle_settings(context: &mut AppContext, settings: BattleSettings) -> FFIOutcome {
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        p.random = Random::new(settings.seed);
        p.battle_settings = settings;
        FFIOutcome::Ok
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_get_stellar_system_parameters(context: &mut AppContext) -> FFIResult<StellarSystemParameters> {
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        return FFIResult::ok(p.stellar_system_parameters.clone());
    }

    FFIResult::unable()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_set_stellar_system_parameters(context: &mut AppContext, parameters: StellarSystemParameters) -> FFIOutcome {
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        p.stellar_system_parameters = parameters;
        FFIOutcome::Ok
    } else {
        FFIOutcome::Unable
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_sandbox_add_warpgate(context: &mut AppContext, faction: Faction) -> FFIOutcome {
    let guard = &mut context.guard;
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
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
pub extern "C" fn ice_sandbox_start_battle(context: &mut AppContext) -> FFIOutcome {
    let guard = &mut context.guard;
    let logger = LoggerRef::new(&context.logger);
    if let Some(GameVariant::Sandbox(p)) = &mut context.game.variant {
        let result = guard.wrap(|| {
            p.start_battle(&logger)
        });
        result.outcome
    } else {
        FFIOutcome::Unable
    }
}

pub struct Sandbox {
    current_battle: Option<Battle>,
    battle_settings: BattleSettings,
    stellar_system_parameters: StellarSystemParameters,
    warpgates: StructVec5<WarpGate>,
    uniqueness_registry: UniquenessRegistry,
    random: Random,
}

impl Sandbox {
    pub fn add_warpgate(&mut self, faction: Faction) -> Result<(), String> {
        let position = WarpGate::generate_position(&mut self.random, &self.stellar_system_parameters);
        let warp_gate = WarpGate::new(position, faction, &mut self.uniqueness_registry);
        self.warpgates.add(warp_gate)
    }

    pub fn start_battle(&mut self, logger: &LoggerRef) -> Result<(), String> {
        if self.current_battle.is_some() {
            return Err("Battle is already active".to_string());
        }

        let stellar_system_info = StellarSystemInfo::new(
            &mut self.random,
            self.stellar_system_parameters.clone(),
            &mut self.uniqueness_registry,
            logger,
        );

        self.current_battle = Some(Battle::new(
            self.battle_settings.clone(),
            stellar_system_info,
            Faction::Red,
            self.warpgates.clone(),
            logger,
        ));

        Ok(())
    }

    pub fn get_current_battle_ref(&self) -> Option<&Battle> { self.current_battle.as_ref() }

    pub fn get_current_battle_mut(&mut self) -> Option<&mut Battle> { self.current_battle.as_mut() }

    pub fn close(&mut self) -> Result<(), String> {
        if let Some(battle) = &mut self.current_battle {
            battle.close()?;
        }
        Ok(())
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        let battle_settings = BattleSettings::default();
        let random = Random::new(battle_settings.seed);
        Self {
            current_battle: None,
            stellar_system_parameters: StellarSystemParameters::default(),
            battle_settings,
            warpgates: StructVec5::default(),
            uniqueness_registry: UniquenessRegistry::default(),
            random,
        }
    }
}

