pub mod game;
pub mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use interoptopus::{ffi_type, ffi_function};
use crate::app::game::GameContext;
use crate::app::utils::guard::Guard;
use crate::app::utils::DeltaTime;
use crate::app::utils::interop_logger::InteropLogger;
use crate::ffi::utils::FFIOutcome;

#[ffi_type(opaque)]
#[derive(Default)]
pub struct AppContext {
    game: GameContext,
    guard: Guard,
    logger: Rc<RefCell<InteropLogger>>
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_app_update(context: &mut AppContext, delta_time: f32) -> FFIOutcome {
    let guard = &mut context.guard;
    let game = &mut context.game;
    let delta = DeltaTime::new(delta_time);

    let result = guard.wrap(|| {
        game.update(delta);
        Ok(())
    });
    result.outcome
}


