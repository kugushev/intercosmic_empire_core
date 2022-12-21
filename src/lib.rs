#![allow(non_snake_case)]

mod steering;

use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function};
use glam::Vec3;


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

    fn map(self) -> Vec3 {
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

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ICEHelloFromRust))
        .register(function!(ICESteeringSeek))
        .inventory()
}
