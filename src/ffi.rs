use std::ffi::CString;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::ptr::{null_mut};
use interoptopus::{ffi_function, ffi_surrogates};
use glam::Vec3;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use crate::steering;
use crate::ffi_ext::*;
use crate::ffi_models::{FFIResult, StellarSystemViewModel};
use crate::game::game_context::GameContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_view_model::BattleViewModel;
use crate::game::battle::models::warp_gate::WarpGate;
use crate::game::core::models::stellar_system::{Planet, StellarSystemId, StellarSystemParameters, Sun};


#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_hello_from_rust(a: i32) -> i32 {
    a + a + 42
}

#[ffi_function]
#[ffi_surrogates(position = "vec3_read_ptr", target = "vec3_read_ptr", current_velocity = "vec3_read_ptr", output = "vec3_read_write_ptr")]
#[no_mangle]
pub extern "C" fn ice_steering_seek(position: &Vec3, target: &Vec3, mass: f32, max_speed: f32,
                                    current_velocity: &Vec3, output: &mut Vec3) -> FFIResult {
    *output = steering::seek(position, target, mass, max_speed, current_velocity);
    FFIResult::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_init_game(context: &mut *mut GameContext) -> FFIResult {
    if !context.is_null() {
        return FFIResult::NotNullPointerError;
    }

    let boxed = Box::new(GameContext::new());

    *context = Box::into_raw(boxed);

    FFIResult::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_close_game(context: &mut *mut GameContext) -> FFIResult {
    if context.is_null() {
        return FFIResult::NullPointerError;
    }

    {
        let context = unsafe { Box::from_raw(*context) };
        drop(context);
    }

    *context = null_mut();

    FFIResult::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_stellar_system(context: &mut GameContext, id: StellarSystemId, sun: Sun, parameters: StellarSystemParameters) {
    // todo: wrap to AsciiError
    context.register_stellar_system(id, sun, parameters);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_planet(context: &mut GameContext, id: StellarSystemId, planet: Planet) {
    // todo: wrap to AsciiError
    context.register_planet(id, planet);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_start_battle(context: &mut GameContext, parameters: BattleParameters) {
    context.start_battle(parameters);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_open_warp_gate(context: &mut GameContext, warp_gate: WarpGate) {
    // todo: return result than unwrap
    context.battle_context.as_mut().unwrap().open_warp_gate(warp_gate);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_finish_battle(context: &mut GameContext) {
    context.finish_battle();
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_update(context: &mut GameContext, delta_time: f32) -> AsciiPointer {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        match &mut context.battle_context {
            None => {}
            Some(ctx) => { ctx.ecs.update(delta_time); }
        }
    }));

    // todo: make it generic
    match result {
        Ok(_) => {
            AsciiPointer::empty()
        }
        Err(panic) => {
            match panic.downcast::<String>() {
                Ok(panic_msg) => {
                    let msg = panic_msg.to_string();
                    context.last_panic = CString::new(msg).unwrap();
                    AsciiPointer::from_cstr(&context.last_panic)
                }
                Err(_) => {
                    AsciiPointer::empty()
                }
            }
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_view_model(context: &mut GameContext) -> &BattleViewModel {
    match &context.battle_context {
        None => { panic!("Oh no!") }
        Some(ctx) => { ctx.get_view_model() }
    }
}


#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_stellar_system_view_model(context: &mut GameContext) -> StellarSystemViewModel {
    // todo: return error than unwrap
    let battle_ctx = context.battle_context.as_ref().unwrap();
    let stellar_system = battle_ctx.get_stellar_system();
    StellarSystemViewModel {
        id: stellar_system.id,
        sun: &stellar_system.sun,
        parameters: &stellar_system.parameters,
        planets: FFISlice::from_slice(&stellar_system.planets[..]),
        warp_gates: FFISlice::from_slice(&battle_ctx.warp_gates[..]),
    }
}


