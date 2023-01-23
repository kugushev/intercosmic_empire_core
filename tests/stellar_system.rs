use std::ops::Index;
use std::ptr;
use glam::Vec3;
use intercosmic_empire::ffi::{ice_battle_update, ice_get_battle_stellar_system_view_model, ice_init_game, ice_register_planet, ice_register_stellar_system, ice_start_battle};
use intercosmic_empire::ffi_models::{FFILog, FFIOutcome, FFIResult, StellarSystemViewModel};
use intercosmic_empire::game::game_context::GameContext;
use intercosmic_empire::game::battle::models::battle_parameters::BattleParameters;
use intercosmic_empire::game::core::models::faction::Faction;
use intercosmic_empire::game::core::models::stellar_system::{Orbit, Planet, PlanetId, PlanetInfo, PlanetSize, Production, StellarSystemId, StellarSystemParameters, Sun};

#[test]
fn battle_planets_production() {
    // arrange
    let mut game_context_ptr: *mut GameContext = ptr::null_mut();
    ice_init_game(&mut game_context_ptr).assert(FFIOutcome::Ok);

    let game_context = unsafe { &mut (*game_context_ptr) };

    let stellar_system_id = StellarSystemId(42);
    ice_register_stellar_system(game_context, stellar_system_id,
                                Sun { position: Vec3::new(1.0, 1.0, 1.0), radius: 1.0 },
                                StellarSystemParameters::default());

    ice_register_planet(game_context, stellar_system_id, Planet {
        position: Default::default(),
        faction: Faction::Green,
        current_product: 0.0,
        info: PlanetInfo {
            id: PlanetId(1),
            orbit: Orbit { radius: 0.0, alpha_rotation: 0.0, beta_rotation: 0.0, start_day: 0 },
            size: PlanetSize::Mercury,
            production: Production { amount_per_second: 2.0, max_product: 10.0 },
        },
    });

    // act
    ice_start_battle(game_context, BattleParameters {
        seed: 42,
        stellar_system_id,
    }).assert(FFIOutcome::Ok);

    // assert
    ice_battle_update(game_context, 1.0, FFILog::default()).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_stellar_system_view_model(game_context);
        let planet = get_single_planet(&view_model);
        assert_eq!(2.0, planet.current_product);
    }

    ice_battle_update(game_context, 0.5, FFILog::default()).assert(FFIOutcome::Ok);

    {
        let view_model = ice_get_battle_stellar_system_view_model(game_context);
        let planet = get_single_planet(&view_model);
        assert_eq!(3.0, planet.current_product);
    }
}

fn get_single_planet<'a>(stellar_system_view_model: &'a FFIResult<StellarSystemViewModel>) -> &'a Planet{
    stellar_system_view_model.outcome.assert(FFIOutcome::Ok);
    assert_eq!(1, stellar_system_view_model.value.planets.len());
    stellar_system_view_model.value.planets.index(0)
}