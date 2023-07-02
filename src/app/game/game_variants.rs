pub mod playground;

use std::fmt::{Debug, Formatter};
use crate::app::game::game_variants::playground::Playground;
use crate::app::utils::DeltaTime;

pub enum GameVariant {
    Playground(Playground)
}

impl GameVariant {
    pub(crate) fn update(&mut self, delta: DeltaTime) {
        match self {
            GameVariant::Playground(playground) => { playground.update(delta); }
        }
    }
}

impl GameVariant {
    pub fn new_playground() -> Self {
        Self::Playground(Playground::default())
    }
}

impl Debug for GameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameVariant::Playground(_) => { write!(f, "Playground")?; }
        };
        Ok(())
    }
}