use std::ptr;
use glam::Vec3;
use interoptopus::patterns::string::AsciiPointer;
use intercosmic_empire::ffi::{ice_battle_update, ice_build_route_add_waypoint, ice_build_route_new, ice_get_battle_view_model, ice_init_game, ice_register_planet, ice_register_stellar_system, ice_spawn_spaceship, ice_start_battle, ice_subscribe_logs};
use intercosmic_empire::ffi_models::{ffi_log_println, FFILog, FFIOutcome, RouteBuildersSource};
use intercosmic_empire::game::battle::battle_params::GAP_BETWEEN_WAYPOINTS;
use intercosmic_empire::game::game_context::GameContext;
use intercosmic_empire::game::battle::models::battle_parameters::BattleParameters;
use intercosmic_empire::game::battle::systems::spaceships::spaceship_movement::SUFFICIENT_DISTANCE_TO_TARGET;
use intercosmic_empire::game::core::models::faction::Faction;
use intercosmic_empire::game::core::models::stellar_system::{Orbit, Planet, PlanetId, PlanetInfo, PlanetSize, Production, StellarSystemId, StellarSystemParameters, Sun};
use intercosmic_empire::game::core::models::stellar_system::spaceport::Spaceport;

#[test]
fn spawn_spaceship_start_translation_is_land() {
    // arrange
    let mut game_context_ptr: *mut GameContext = ptr::null_mut();
    ice_init_game(&mut game_context_ptr).assert(FFIOutcome::Ok);
    let game_context = unsafe { &mut (*game_context_ptr) };

    ice_subscribe_logs(game_context, ffi_log_println());

    let stellar_system_id = StellarSystemId(42);
    ice_register_stellar_system(game_context, stellar_system_id,
                                Sun { position: Vec3::new(0.0, 1.5, 0.5), radius: 0.05 },
                                StellarSystemParameters::default());

    const POSITION1: Vec3 = Vec3 { x: 0.0, y: 1.7, z: 0.5 };
    const PLANET1_ID: i32 = 1;
    ice_register_planet(game_context, stellar_system_id, create_earthlike(POSITION1, PLANET1_ID))
        .assert(FFIOutcome::Ok);

    const POSITION2: Vec3 = Vec3 { x: 0.1, y: 1.5, z: 0.9 };
    ice_register_planet(game_context, stellar_system_id, create_earthlike(POSITION2, 2))
        .assert(FFIOutcome::Ok);

    // act
    ice_start_battle(game_context, BattleParameters {
        seed: 42,
        stellar_system_id,
    }).assert(FFIOutcome::Ok);

    let mut builder_id = 0;
    ice_build_route_new(game_context, RouteBuildersSource::Ai, &POSITION1,
                        SPACEPORT.clone(), &mut builder_id);
    let waypoints = waypoints_generator(POSITION1, vec![POSITION2].into_iter());
    for waypoint in waypoints {
        ice_build_route_add_waypoint(game_context, RouteBuildersSource::Ai, builder_id, &waypoint);
    }
    ice_spawn_spaceship(game_context, RouteBuildersSource::Ai,
                        builder_id, &POSITION2, &SPACEPORT,
                        &FACTION, PLANET1_ID);

    // assert

    let mut elapsed = 0.0;
    while elapsed < 10.0 {
        ice_battle_update(game_context, DELTA_TIME_120_FPS).assert(FFIOutcome::Ok);
        elapsed += DELTA_TIME_120_FPS;

        let battle_vm = ice_get_battle_view_model(game_context);
        let spaceship = battle_vm.value.spaceships.first();
        match spaceship {
            None => {}
            Some(s) => {
                if (s.position - POSITION2).length() < SPACEPORT.orbit_radius + GAP_BETWEEN_WAYPOINTS + SUFFICIENT_DISTANCE_TO_TARGET {
                    return;
                }
            }
        }
    }

    let battle_vm = ice_get_battle_view_model(game_context);
    let spaceship = battle_vm.value.spaceships.first().unwrap();
    assert!(false, "Shit! {}", spaceship.position)
}

static DELTA_TIME_120_FPS: f32 = 0.008;
static SPACEPORT: Spaceport = Spaceport { surface_radius: 0.01953, orbit_radius: 0.063 };
static FACTION: Faction = Faction::Green;

fn create_earthlike(position: Vec3, id: i32) -> Planet {
    Planet {
        position,
        faction: FACTION.clone(),
        current_product: 50.0,
        under_siege: false,
        info: PlanetInfo {
            id: PlanetId(id),
            orbit: Orbit { radius: 0.0, alpha_rotation: 0.0, beta_rotation: 0.0, start_day: 0 },
            size: PlanetSize::Earth,
            production: Production { amount_per_second: 0.0, max_product: 0.0 },
            spaceport: SPACEPORT.clone(),
        },
    }
}

fn waypoints_generator(start: Vec3, anchors: impl Iterator<Item=Vec3>) -> Vec<Vec3> {
    let mut waypoints = vec![];

    let current = start;
    for next in anchors {
        let direction = next - current;
        let mut offset_length = 0.0;
        while offset_length < direction.length() {
            let offset_direction = direction.normalize() * offset_length;
            let offset_position = current + offset_direction;

            let waypoint_direction = (next - offset_position).normalize() * GAP_BETWEEN_WAYPOINTS;
            let waypoint_position = offset_position + waypoint_direction;

            waypoints.push(waypoint_position);

            offset_length += waypoint_direction.length();

            // check on Null Vector
            if waypoint_direction.length() < GAP_BETWEEN_WAYPOINTS * 0.01 {
                break;
            }
        }
        waypoints.push(next);
    }

    waypoints
}