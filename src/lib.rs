use interoptopus::{Inventory, InventoryBuilder, function};

mod ffi;
mod steering;
mod ffi_ext;
mod game;
mod ffi_models;

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ffi::ICEHelloFromRust))
        .register(function!(ffi::ICESteeringSeek))
        .register(function!(ffi::ICEInitGame))
        .register(function!(ffi::ICECloseGame))
        .register(function!(ffi::ICEInitBattle))
        .register(function!(ffi::ICEBattleUpdate))
        .register(function!(ffi::ICEGetBattleViewModel))
        .inventory()
}
