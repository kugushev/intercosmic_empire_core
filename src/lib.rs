use interoptopus::{function, Inventory, InventoryBuilder};

pub mod old_ffi;
pub mod steering;
pub mod old_ffi_ext;
pub mod old;
pub mod old_ffi_models;
pub mod ffi;
pub mod app;

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ffi::ice_init_app))
        .register(function!(ffi::ice_close_app))
        .register(function!(app::ice_app_update))
        .register(function!(app::game::ice_app_start_playground))
        .register(function!(app::game::game_variants::playground::ice_game_playground_get_battle_settings))
        .register(function!(app::game::game_variants::playground::ice_game_playground_set_battle_settings))
        .register(function!(app::game::game_variants::playground::ice_game_playground_get_stellar_system_parameters))
        .register(function!(app::game::game_variants::playground::ice_game_playground_set_stellar_system_parameters))
        .register(function!(app::game::game_variants::playground::ice_game_playground_add_warpgate))
        .register(function!(app::game::game_variants::playground::ice_game_playground_start))

        // .register(function!(old_ffi::ice_hello_from_rust))
        // .register(function!(old_ffi::ice_steering_seek))
        // .register(function!(old_ffi::ice_init_game))
        // .register(function!(old_ffi::ice_close_game))
        // .register(function!(old_ffi::ice_get_last_error))
        // .register(function!(old_ffi::ice_subscribe_logs))
        // .register(function!(old_ffi::ice_toggle_trace))
        // .register(function!(old_ffi::ice_register_stellar_system))
        // .register(function!(old_ffi::ice_register_planet))
        // .register(function!(old_ffi::ice_start_battle))
        // .register(function!(old_ffi::ice_battle_open_warp_gate))
        // .register(function!(old_ffi::ice_finish_battle))
        // .register(function!(old_ffi::ice_battle_update))
        // .register(function!(old_ffi::ice_get_battle_view_model))
        // .register(function!(old_ffi::ice_get_battle_stellar_system_view_model))
        // .register(function!(old_ffi::ice_build_route_new))
        // .register(function!(old_ffi::ice_build_route_add_waypoint))
        // .register(function!(old_ffi::ice_spawn_spaceship))
        .inventory()
}