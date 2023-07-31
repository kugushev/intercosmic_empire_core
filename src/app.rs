pub mod game;
pub mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use interoptopus::ffi_type;
use crate::app::game::GameContext;
use crate::app::utils::guard::Guard;
use crate::app::utils::interop_logger::InteropLogger;

#[ffi_type(opaque)]
pub struct AppContext {
    game: GameContext,
    guard: Guard,
    logger: Rc<RefCell<InteropLogger>>,
    app_settings: AppSettings,
}

#[ffi_type]
#[repr(C)]
#[derive(Default)]
pub struct AppSettings {
    pub flat_mode: bool,
}

impl AppContext {
    pub fn new(app_settings: AppSettings) -> Self {
        Self {
            game: GameContext::default(),
            guard: Guard::default(),
            logger: Rc::<RefCell<InteropLogger>>::default(),
            app_settings,
        }
    }
}

