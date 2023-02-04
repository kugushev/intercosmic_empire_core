use crate::ffi_ext::*;
use crate::ffi_models::{BattleStateViewModel, FFILog, FFIOutcome, FFIResult, StellarSystemViewModel};
use crate::game::battle::components::stellar::warp_gate::WarpGate;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::core::models::stellar_system::{
    Planet, StellarSystemId, StellarSystemParameters, Sun,
};
use crate::game::game_context::GameContext;
use crate::steering;
use glam::Vec3;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{ffi_function, ffi_surrogates};
use std::ptr::null_mut;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_hello_from_rust(a: i32) -> i32 {
    a + a + 42
}

#[ffi_function]
#[ffi_surrogates(
position = "vec3_read_ptr",
target = "vec3_read_ptr",
current_velocity = "vec3_read_ptr",
output = "vec3_read_write_ptr"
)]
#[no_mangle]
pub extern "C" fn ice_steering_seek(
    position: &Vec3,
    target: &Vec3,
    mass: f32,
    max_speed: f32,
    current_velocity: &Vec3,
    output: &mut Vec3,
) {
    *output = steering::seek(position, target, mass, max_speed, current_velocity);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_init_game(context: &mut *mut GameContext) -> FFIOutcome {
    if !context.is_null() {
        return FFIOutcome::Unable;
    }

    let boxed = Box::new(GameContext::new());

    *context = Box::into_raw(boxed);

    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_close_game(context: &mut *mut GameContext) -> FFIOutcome {
    if context.is_null() {
        return FFIOutcome::Unable;
    }

    {
        let context = unsafe { Box::from_raw(*context) };
        drop(context);
    }

    *context = null_mut();

    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_last_error(context: &GameContext) -> AsciiPointer {
    AsciiPointer::from_cstr(&context.last_error_msg)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_last_log(context: &GameContext) -> AsciiPointer {
    AsciiPointer::from_cstr(&context.last_log_msg)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_subscribe_log_signal(context: &mut GameContext, log_delegate: FFILog) {
    context.log_signal_delegate = Some(log_delegate);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_stellar_system(
    context: &mut GameContext,
    id: StellarSystemId,
    sun: Sun,
    parameters: StellarSystemParameters,
) -> FFIOutcome {
    context.guard(|ctx| ctx.register_stellar_system(id, sun, parameters))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_planet(
    context: &mut GameContext,
    id: StellarSystemId,
    planet: Planet,
) -> FFIOutcome {
    context.guard(|ctx| ctx.register_planet(id, planet))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_start_battle(
    context: &mut GameContext,
    parameters: BattleParameters,
) -> FFIOutcome {
    context.guard(|ctx| ctx.start_battle(parameters))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_open_warp_gate(
    context: &mut GameContext,
    warp_gate: WarpGate,
) -> FFIOutcome {
    if context.battle_context.is_none() {
        return FFIOutcome::Unable;
    }

    context.guard(|ctx| {
        ctx.battle_context
            .as_mut()
            .unwrap()
            .open_warp_gate(warp_gate);
        Ok(())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_finish_battle(context: &mut GameContext) {
    context.finish_battle();
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_update(
    context: &mut GameContext,
    delta_time: f32,
    log: FFILog,
) -> FFIOutcome {
    if context.battle_context.is_none() {
        return FFIOutcome::Unable;
    }

    context.guard(|ctx| {
        ctx.battle_context
            .as_mut()
            .unwrap()
            .ecs
            .update(delta_time, log);
        Ok(())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_view_model(
    context: &mut GameContext,
) -> FFIResult<BattleStateViewModel> {
    match context.battle_context.as_ref() {
        Some(battle_ctx) => {
            let view_model = battle_ctx.get_view_model();
            FFIResult::ok(BattleStateViewModel::from(view_model))
        }
        None => FFIResult::unable(),
    }
}


#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_stellar_system_view_model(
    context: &mut GameContext,
) -> FFIResult<StellarSystemViewModel> {
    match context.battle_context.as_ref() {
        Some(battle_ctx) => {
            let stellar_system = battle_ctx.get_stellar_system();

            FFIResult::ok(StellarSystemViewModel {
                id: stellar_system.id,
                sun: &stellar_system.sun,
                parameters: &stellar_system.parameters,
                planets: FFISlice::from_slice(&stellar_system.planets[..]),
            })
        }
        None => FFIResult::unable(),
    }
}
