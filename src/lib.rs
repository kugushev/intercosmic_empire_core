#![allow(non_snake_case)]

mod steering;
mod obstacle;

use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function};
use glam::Vec3;
use interoptopus::patterns::slice::FFISlice;
use crate::obstacle::Obstacle;


#[ffi_type]
#[repr(C)]
pub struct ICEVector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl ICEVector3 {
    fn new(vector: Vec3) -> ICEVector3 {
        ICEVector3 {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }

    fn map(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICEHelloFromRust(a: i32) -> i32 {
    a + a + 42
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICESteeringSeek(position: ICEVector3, target: ICEVector3, mass: f32, maxSpeed: f32,
                                  currentVelocity: ICEVector3) -> ICEVector3 {
    let result = steering::seek(position.map(), target.map(), mass, maxSpeed, currentVelocity.map());
    return ICEVector3::new(result);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ICESteeringAvoid(obstaclesSlice: FFISlice<Obstacle>, position: ICEVector3, mass: f32, maxSpeed: f32, deltaTime: f32,
                                   currentVelocity: ICEVector3) -> ICEVector3 {
    let obstacles = obstaclesSlice.as_slice();

    let result = steering::avoid(obstacles, position.map(), mass, maxSpeed, deltaTime, currentVelocity.map());
    return ICEVector3::new(result);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ffi_slice_1(ffi_slice: FFISlice<u32>) -> u32 {
    ffi_slice.as_slice().len() as u32
}


pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ICEHelloFromRust))
        .register(function!(ICESteeringSeek))
        .register(function!(ICESteeringAvoid))
        .inventory()
}
