use interoptopus::{ffi_function, ffi_type, Inventory, InventoryBuilder, function};

#[ffi_type]
#[repr(C)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn hello_from_rust(a: i32) -> i32 {
    a + a + 42
}

// #[ffi_function]
// #[no_mangle]
// pub extern "C" fn kaw_steering_seek(position: Vector3, target: Vector3, mass: f32, max_speed: f32,
//                                     current_velocity: Vector3) -> Vector3 {
//     return Vector3 { x: 0.1, y: 0.2, z: 0.3 };
// }

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        //.register(function!(kaw_steering_seek))
        .register(function!(hello_from_rust))
        .inventory()
}
