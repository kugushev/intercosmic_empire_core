pub mod sandbox;

use std::fmt::{Debug, Formatter};
use crate::app::game::battle::Battle;
use crate::app::game::game_variants::sandbox::Sandbox;

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