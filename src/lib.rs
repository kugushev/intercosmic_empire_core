use interoptopus::{function, Inventory, InventoryBuilder};

pub mod ffi;
pub mod steering;
pub mod ffi_ext;
pub mod game;
pub mod ffi_models;

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ffi::ice_hello_from_rust))
        .register(function!(ffi::ice_steering_seek))
        .register(function!(ffi::ice_init_game))
        .register(function!(ffi::ice_close_game))
        .register(function!(ffi::ice_get_last_error))
        .register(function!(ffi::ice_get_last_log))
        .register(function!(ffi::ice_subscribe_log_signal))
        .register(function!(ffi::ice_register_stellar_system))
        .register(function!(ffi::ice_register_planet))
        .register(function!(ffi::ice_start_battle))
        .register(function!(ffi::ice_battle_open_warp_gate))
        .register(function!(ffi::ice_finish_battle))
        .register(function!(ffi::ice_battle_update))
        .register(function!(ffi::ice_get_battle_view_model))
        .register(function!(ffi::ice_get_battle_stellar_system_view_model))
        .inventory()
}
