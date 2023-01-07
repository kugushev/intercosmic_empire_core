#![allow(non_snake_case)]

use std::ffi::CString;
use std::panic;
use std::panic::AssertUnwindSafe;
use std::ptr::{null_mut};
use interoptopus::{ffi_function, ffi_type, ffi_surrogates};
use glam::Vec3;
use interoptopus::patterns::string::AsciiPointer;
use crate::steering;
use crate::ffi_ext::*;
use crate::ffi_models::GameContext;
use crate::game::battle::models::battle_parameters::BattleParameters;
use crate::game::battle::models::battle_view_model::BattleViewModel;

#[ffi_type]
#[repr(C)]
pub enum FFIResult {
    Ok,
    NullPointerError,
    NotNullPointerError,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICEHelloFromRust(a: i32) -> i32 {
    a + a + 42
}

#[ffi_function]
#[ffi_surrogates(position = "vec3_read_ptr", target = "vec3_read_ptr", currentVelocity = "vec3_read_ptr", output = "vec3_read_write_ptr")]
#[no_mangle]
pub extern "C" fn ICESteeringSeek(position: &Vec3, target: &Vec3, mass: f32, maxSpeed: f32,
                                  currentVelocity: &Vec3, output: &mut Vec3) -> FFIResult {
    *output = steering::seek(position, target, mass, maxSpeed, currentVelocity);
    FFIResult::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICEInitGame(context: &mut *mut GameContext) -> FFIResult {
    if !context.is_null() {
        return FFIResult::NotNullPointerError;
    }

    let boxed = Box::new(GameContext::new());

    *context = Box::into_raw(boxed);

    FFIResult::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICECloseGame(context: &mut *mut GameContext) -> FFIResult {
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
pub extern "C" fn ICEInitBattle(context: &mut GameContext, parameters: BattleParameters) {
    context.battle_context.init(parameters);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICEBattleUpdate(context: &mut GameContext, deltaTime: f32) -> AsciiPointer {
    let result = panic::catch_unwind(AssertUnwindSafe(|| {
        context.battle_context.ecs.update(deltaTime);
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
pub extern "C" fn ICEGetBattleViewModel(context: &mut GameContext) -> &BattleViewModel {
    &context.battle_context.get_view_model()
}


