pub mod core;
pub mod game_variants;
pub mod battle;

use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::game_variants::GameVariant;
use crate::app::utils::DeltaTime;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_app_start_playground(context: &mut AppContext) -> FFIResult<()> {
    let guard = &mut context.guard;
    let game = &mut context.game;
    guard.wrap(|| {
        game.start_playground()
    })
}

#[derive(Default)]
pub struct GameContext {
    pub variant: Option<GameVariant>,
}

impl GameContext {
    pub fn update(&mut self, delta: DeltaTime) {
        if let Some(game) = &mut self.variant {
            game.update(delta);
        }
    }

    pub fn start_playground(&mut self) -> Result<(), String> {
        match &self.variant {
            None => {
                self.variant = Some(GameVariant::new_playground());
                Ok(())
            }
            Some(variant) => {
                Err(format!("The other variant is already active {variant:?}"))
            }
        }
    }
}

