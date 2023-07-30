pub mod sandbox;

use std::fmt::{Debug, Formatter};
use interoptopus::{ffi_function, ffi_type};
use crate::app::AppContext;
use crate::app::game::battle::Battle;
use crate::app::game::game_variants::sandbox::Sandbox;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_current_game_variant(context: &mut AppContext) -> FFIResult<FFIGameVariant> {
    let guard = &mut context.guard;
    let game = &mut context.game;
    guard.wrap(|| {
        Ok(match game.variant {
            None => FFIGameVariant::None,
            Some(GameVariant::Sandbox(_)) => FFIGameVariant::Sandbox,
        })
    })
}

#[ffi_type]
#[repr(C)]
pub enum FFIGameVariant {
    None,
    Sandbox,
}

pub enum GameVariant {
    Sandbox(Sandbox)
}

impl GameVariant {
    pub fn get_current_battle_ref(&self) -> Option<&Battle> {
        match self {
            GameVariant::Sandbox(sandbox) => {
                sandbox.get_current_battle_ref()
            }
        }
    }

    pub fn get_current_battle_mut(&mut self) -> Option<&mut Battle> {
        match self {
            GameVariant::Sandbox(sandbox) => {
                sandbox.get_current_battle_mut()
            }
        }
    }
}

impl GameVariant {
    pub fn new_sandbox() -> Self {
        Self::Sandbox(Sandbox::default())
    }
}

impl Debug for GameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameVariant::Sandbox(_) => { write!(f, "sandbox")?; }
        };
        Ok(())
    }
}