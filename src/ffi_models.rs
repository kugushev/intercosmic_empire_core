use std::ffi::CString;
use interoptopus::ffi_type;
use crate::game::battle::battle_context::BattleContext;

#[ffi_type(opaque)]
pub struct GameContext {
    pub(crate) battle_context: BattleContext,
    pub(crate) last_panic: CString
    // todo: add game_view_model
}

impl GameContext {
    pub(crate) fn new() -> GameContext {
        GameContext {
            battle_context: BattleContext::new(),
            last_panic: CString::default()
        }
    }
}

