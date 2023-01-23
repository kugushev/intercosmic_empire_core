use std::collections::HashMap;
use std::ffi::CString;
use std::panic;
use interoptopus::ffi_type;
use crate::ffi_models::{FFIOutcome, FFILog};
use crate::game::battle::battle_context::BattleContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::core::models::stellar_system::{Planet, StellarSystem, StellarSystemId, StellarSystemParameters, Sun};

#[ffi_type(opaque)]
pub struct GameContext {
    pub(crate) battle_context: Option<BattleContext>,
    stellar_map: HashMap<StellarSystemId, StellarSystem>,
    pub(crate) last_error_msg: CString,
    pub(crate) last_log_msg: CString,
    pub log_signal_delegate: Option<FFILog>,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            battle_context: None,
            stellar_map: HashMap::new(),
            last_error_msg: CString::default(),
            last_log_msg: CString::default(),
            log_signal_delegate: None
        }
    }

    pub(crate) fn start_battle(&mut self, parameters: BattleParameters) -> Result<(), String> {
        assert!(self.battle_context.is_none());
        let stellar_system = self.stellar_map.get(&parameters.stellar_system_id)
            .ok_or("Stellar system not found".to_string())?;
        self.battle_context = Some(BattleContext::new(parameters, (*stellar_system).clone()));
        Ok(())
    }

    pub(crate) fn finish_battle(&mut self) {
        self.battle_context = None;
    }

    pub(crate) fn register_stellar_system(&mut self, id: StellarSystemId, sun: Sun, parameters: StellarSystemParameters) -> Result<(), String> {
        if self.stellar_map.contains_key(&id) {
            return Err("Stellar map is already exist".to_string());
        }

        let stellar_system = StellarSystem {
            id,
            sun,
            parameters,
            planets: vec![],
        };

        self.stellar_map.insert(id, stellar_system);

        Ok(())
    }

    pub(crate) fn register_planet(&mut self, stellar_system_id: StellarSystemId, planet: Planet) -> Result<(), String> {
        let stellar_system = self.stellar_map.get_mut(&stellar_system_id)
            .ok_or("Stellar system not found".to_string())?;

        stellar_system.planets.push(planet);

        Ok(())
    }
}


// utils
impl GameContext {
    pub(crate) fn wrap_error_msg(&mut self, msg: String) {
        self.last_error_msg = CString::new(msg).unwrap();
    }

    pub(crate) fn guard<F>(&mut self, f: F) -> FFIOutcome
        where F: FnOnce(&mut GameContext) -> Result<(), String>
    {
        let result_result = panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            f(self)
        }));

        match result_result {
            Ok(r) => match r {
                Ok(_) => { FFIOutcome::Ok }
                Err(msg) => {
                    self.wrap_error_msg(msg);
                    FFIOutcome::Error
                }
            },
            Err(panic) => {
                match panic.downcast::<String>() {
                    Ok(panic_msg) => {
                        self.wrap_error_msg(*panic_msg)
                    }
                    Err(_) => {
                        self.wrap_error_msg("Panic doesn't have appropriate message".to_string())
                    }
                }
                FFIOutcome::Error
            }
        }
    }
}
