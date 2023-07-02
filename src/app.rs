pub mod game;
pub mod guard;
pub mod utils;

use interoptopus::{ffi_type, ffi_function};
use crate::app::game::GameContext;
use crate::app::guard::Guard;
use crate::app::utils::DeltaTime;
use crate::ffi::utils::FFIOutcome;

#[ffi_type(opaque)]
#[derive(Default)]
pub struct AppContext {
    game: GameContext,
    guard: Guard,
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


