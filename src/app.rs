pub mod game;
pub mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use interoptopus::ffi_type;
use crate::app::game::GameContext;
use crate::app::utils::guard::Guard;
use crate::app::utils::interop_logger::InteropLogger;

#[ffi_type(opaque)]
#[derive(Default)]
pub struct AppContext {
    game: GameContext,
    guard: Guard,
    logger: Rc<RefCell<InteropLogger>>,
}


