use interoptopus::ffi_type;
use crate::app::game::battle::Battle;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::GameContext;

#[ffi_type]
#[repr(C)]
pub struct BattleViewModel<'a> {
    pub stellar_system: &'a StellarSystem,
}

impl<'a> BattleViewModel<'a> {
    pub fn current(game_context: &'a GameContext) -> Result<Self, String> {
        let battle = Battle::current_ref(game_context);
        match battle {
            Some(b) => {
                Ok(Self {
                    stellar_system: &b.stellar_system
                })
            }
            None => { Err("Current battle not found".to_string()) }
        }
    }
}