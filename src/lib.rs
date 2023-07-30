use interoptopus::{function, Inventory, InventoryBuilder};

pub mod ffi;
pub mod app;

pub fn my_inventory() -> Inventory {
    InventoryBuilder::new()
        .register(function!(ffi::ice_init_app))
        .register(function!(ffi::ice_close_app))
        .register(function!(app::game::core::stellar_system::planet_size::ice_registry_get_planet_ratio))
        .register(function!(app::utils::guard::ice_get_last_exception))
        .register(function!(app::utils::interop_logger::ice_subscribe_logs))
        .register(function!(app::utils::interop_logger::ice_toggle_trace))
        // game
        .register(function!(app::game::game_variants::ice_get_current_game_variant))
        // sandbox
        .register(function!(app::game::ice_start_sandbox))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_close))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_get_battle_settings))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_set_battle_settings))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_get_stellar_system_parameters))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_set_stellar_system_parameters))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_add_warpgate))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_start_battle))
        .register(function!(app::game::game_variants::sandbox::ice_sandbox_stop_battle))
        // battle
        .register(function!(app::game::battle::ice_battle_is_active))
        .register(function!(app::game::battle::ice_battle_update))
        .register(function!(app::game::battle::ice_battle_get_vm))
        .register(function!(app::game::battle::entities::fleet::fleet_view_model::ice_battle_fleet_get_vm))
        .register(function!(app::game::battle::entities::fleet::fleet_spawn::ice_battle_fleet_spawn_begin))
        .register(function!(app::game::battle::entities::fleet::fleet_spawn::ice_battle_fleet_spawn_add_waypoint))
        .register(function!(app::game::battle::entities::fleet::fleet_spawn::ice_battle_fleet_spawn_finish))
        // utils
        .register(function!(app::utils::quat_extra::ice_test_quat_look_rotation))
        .inventory()
}