use interoptopus::{Inventory, InventoryBuilder, function};

mod ffi;
mod steering;
mod ffi_ext;

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ffi::ICEHelloFromRust))
        .register(function!(ffi::ICESteeringSeek))
        .inventory()
}
