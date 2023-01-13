use std::collections::HashMap;
use std::ffi::CString;
use interoptopus::ffi_type;
use crate::game::battle::battle_context::BattleContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::core::models::stellar_system::{Planet, StellarSystem, StellarSystemId, StellarSystemParameters, Sun};

#[ffi_type(opaque)]
pub struct GameContext {
    pub(crate) battle_context: Option<BattleContext>,
    stellar_map: HashMap<StellarSystemId, StellarSystem>,
    pub(crate) last_panic: CString,
}

impl GameContext {
    pub(crate) fn new() -> GameContext {
        GameContext {
            battle_context: None,
            stellar_map: HashMap::new(),
            last_panic: CString::default(),
        }
    }

    pub(crate) fn start_battle(&mut self, parameters: BattleParameters) -> Result<(), &str> {
        assert!(self.battle_context.is_none());
        let stellar_system = self.stellar_map.get(&parameters.stellar_system_id)
            .ok_or("Stellar system not found")?;
        self.battle_context = Some(BattleContext::new(parameters, (*stellar_system).clone()));
        Ok(())
    }

    pub(crate) fn finish_battle(&mut self) {
        self.battle_context = None;
    }

    pub(crate) fn register_stellar_system(&mut self, id: StellarSystemId, sun: Sun, parameters: StellarSystemParameters) -> Result<(), &str> {
        if self.stellar_map.contains_key(&id) {
            return Err("Stellar map is already exist")
        }

        let stellar_system = StellarSystem {
            id,
            sun,
            parameters,
            planets: vec![]
        };

        self.stellar_map.insert(id, stellar_system);

        Ok(())
    }

    pub(crate) fn register_planet(&mut self, stellar_system_id: StellarSystemId, planet: Planet) -> Result<(), &str> {
        let stellar_system = self.stellar_map.get_mut(&stellar_system_id).ok_or("Stellar system not found")?;

        stellar_system.planets.push(planet);

        Ok(())
    }

    // pub(crate) fn get_stellar_system_clone(&self, id: StellarSystemId) -> Option<StellarSystem> {
    //     if self.stellar_map. { }
    // }
}

