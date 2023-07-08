use std::ops::Index;
use std::ptr;
use glam::Vec3;
use intercosmic_empire::old_ffi::{ice_battle_open_warp_gate, ice_battle_update_old, ice_get_battle_stellar_system_view_model, ice_get_battle_view_model, ice_init_game, ice_register_planet, ice_register_stellar_system, ice_start_battle, ice_subscribe_logs_old};
use intercosmic_empire::old_ffi_models::{BattleStateViewModel, ffi_log_println, FFIOutcome, FFIResult, StellarSystemViewModel};
use intercosmic_empire::old::game_context::GameContext;
use intercosmic_empire::old::battle::models::battle_parameters::BattleParameters;
use intercosmic_empire::old::battle::models::warp_gate::WarpGate;
use intercosmic_empire::old::core::models::faction::Faction;
use intercosmic_empire::old::core::models::stellar_system::{Orbit, Planet, PlanetId, PlanetInfo, PlanetSize, Production, StellarSystemId, StellarSystemParameters, Sun};
use intercosmic_empire::old::core::models::stellar_system::spaceport::Spaceport;

#[test]
fn battle_planet_production() {
    do_battle_planets_production(Faction::Green, 2.0, 3.0);
}

#[test]
fn battle_abandoned_planet_no_production() {
    do_battle_planets_production(Faction::Grey, 0.0, 0.0);
}

fn do_battle_planets_production(faction: Faction, product1: f32, product2: f32) {
    // arrange
    let mut game_context_ptr: *mut GameContext = ptr::null_mut();
    ice_init_game(&mut game_context_ptr).assert(FFIOutcome::Ok);
    let game_context = unsafe { &mut (*game_context_ptr) };

    ice_subscribe_logs_old(game_context, ffi_log_println());

    let stellar_system_id = StellarSystemId(42);
    ice_register_stellar_system(game_context, stellar_system_id,
                                Sun { position: Vec3::new(1.0, 1.0, 1.0), radius: 1.0 },
                                StellarSystemParameters::default());

    ice_register_planet(game_context, stellar_system_id, Planet {
        position: Default::default(),
        faction,
        current_product: 0.0,
        under_siege: false,
        info: PlanetInfo {
            id: PlanetId(1),
            orbit: Orbit { radius: 0.0, alpha_rotation: 0.0, beta_rotation: 0.0, start_day: 0 },
            size: PlanetSize::Mercury,
            production: Production { amount_per_second: 2.0, max_product: 10.0 },
            spaceport: Spaceport { surface_radius: 0.0, orbit_radius: 0.0 },
        },
    }).assert(FFIOutcome::Ok);

    // act
    ice_start_battle(game_context, BattleParameters {
        seed: 42,
        stellar_system_id,
    }).assert(FFIOutcome::Ok);

    // assert
    ice_battle_update_old(game_context, 1.0).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_stellar_system_view_model(game_context);
        let planet = get_single_planet(&view_model);
        assert_eq!(product1, planet.current_product);
    }

    ice_battle_update_old(game_context, 0.5).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_stellar_system_view_model(game_context);
        let planet = get_single_planet(&view_model);
        assert_eq!(product2, planet.current_product);
    }
}

#[test]
fn battle_warpgates_production() {
    // arrange
    let mut game_context_ptr: *mut GameContext = ptr::null_mut();
    ice_init_game(&mut game_context_ptr).assert(FFIOutcome::Ok);

    let game_context = unsafe { &mut (*game_context_ptr) };

    ice_subscribe_logs_old(game_context, ffi_log_println());

    let stellar_system_id = StellarSystemId(42);
    ice_register_stellar_system(game_context, stellar_system_id,
                                Sun { position: Vec3::new(1.0, 1.0, 1.0), radius: 1.0 },
                                StellarSystemParameters::default());

    // act
    ice_start_battle(game_context, BattleParameters {
        seed: 42,
        stellar_system_id,
    }).assert(FFIOutcome::Ok);

    let mut warp_gate_id = 0;
    ice_battle_open_warp_gate(game_context, WarpGate {
        position: Default::default(),
        faction: Faction::Green,
        current_product: 0.0,
        production: Production { amount_per_second: 2.0, max_product: 10.0 },
        spaceport: Spaceport { surface_radius: 0.0, orbit_radius: 0.0 },
    }, &mut warp_gate_id).assert(FFIOutcome::Ok);

    // assert
    ice_battle_update_old(game_context, 1.0).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_view_model(game_context);
        let warp_gate = get_single_warp_gate(&view_model);
        assert_eq!(2.0, warp_gate.current_product);
    }

    ice_battle_update_old(game_context, 0.5).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_view_model(game_context);
        let warp_gate = get_single_warp_gate(&view_model);
        assert_eq!(3.0, warp_gate.current_product);
    }
}

fn get_single_planet<'a>(stellar_system_view_model: &'a FFIResult<StellarSystemViewModel>) -> &'a Planet {
    stellar_system_view_model.outcome.assert(FFIOutcome::Ok);
    assert_eq!(1, stellar_system_view_model.value.planets.len());
    stellar_system_view_model.value.planets.index(0)
}

fn get_single_warp_gate<'a>(view_model: &'a FFIResult<BattleStateViewModel>) -> &'a WarpGate {
    view_model.outcome.assert(FFIOutcome::Ok);
    assert_eq!(1, view_model.value.warp_gates.len());
    view_model.value.warp_gates.index(0)
}