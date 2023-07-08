pub mod core;
pub mod game_variants;
pub mod battle;

use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::game_variants::GameVariant;
use crate::ffi::utils::FFIOutcome;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_start_sandbox(context: &mut AppContext) -> FFIOutcome {
    let guard = &mut context.guard;
    let game = &mut context.game;
    let result = guard.wrap(|| {
        game.start_sandbox()?;
        Ok(())
    });
    result.outcome
}

#[derive(Default)]
pub struct GameContext {
    pub variant: Option<GameVariant>,
}

impl GameContext {
    pub fn start_sandbox(&mut self) -> Result<(), String> {
        match &self.variant {
            None => {
                self.variant = Some(GameVariant::new_sandbox());
                Ok(())
            }
            Some(variant) => {
                Err(format!("The other variant is already active {variant:?}"))
            }
        }
    }
}

