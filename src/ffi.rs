#![allow(non_snake_case)]

use interoptopus::{ffi_function, ffi_type, ffi_surrogates};
use glam::Vec3;
use crate::steering;
use crate::ffi_ext::vec3_read_ptr;
use crate::ffi_ext::vec3_read_write_ptr;

#[ffi_type]
#[repr(C)]
pub enum FFIResult {
    Ok
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