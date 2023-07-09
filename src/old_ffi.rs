use crate::old_ffi_ext::*;
use crate::old_ffi_models::{
    BattleStateViewModel, FFILog, FFIOutcome, FFIResult, RouteBuildersSource,
    StellarSystemViewModel,
};
use crate::old::battle::models::battle_parameters::BattleParameters;
use crate::old::battle::models::warp_gate::WarpGate;
use crate::old::core::models::stellar_system::{
    Planet, StellarSystemId, StellarSystemParameters, Sun,
};
use crate::old::game_context::GameContext;
use crate::{steering};
use glam::Vec3;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use interoptopus::{ffi_function, ffi_surrogates};
use std::ptr::null_mut;
use crate::old::battle::models::route::RouteBuilder;
use crate::old::core::models::faction::Faction;
use crate::old::core::models::spaceships::spaceship_mark::SpaceshipMark;
use crate::old::core::models::stellar_system::spaceport::Spaceport;


#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_hello_from_rust(a: i32) -> i32 {
    a + a + 42
}

#[ffi_function]
#[ffi_surrogates(
position = "vec3_read_ptr",
target = "vec3_read_ptr",
current_velocity = "vec3_read_ptr",
output = "vec3_read_write_ptr"
)]
#[no_mangle]
pub extern "C" fn ice_steering_seek(
    position: &Vec3,
    target: &Vec3,
    mass: f32,
    max_speed: f32,
    current_velocity: &Vec3,
    output: &mut Vec3,
) {
    *output = steering::seek(position, target, mass, max_speed, current_velocity);
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_init_game(context: &mut *mut GameContext) -> FFIOutcome {
    if !context.is_null() {
        return FFIOutcome::Unable;
    }

    let boxed = Box::new(GameContext::new());

    *context = Box::into_raw(boxed);

    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_close_game(context: &mut *mut GameContext) -> FFIOutcome {
    if context.is_null() {
        return FFIOutcome::Unable;
    }

    {
        let context = unsafe { Box::from_raw(*context) };
        drop(context);
    }

    *context = null_mut();

    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_last_error(context: &GameContext) -> AsciiPointer {
    AsciiPointer::from_cstr(&context.last_error_msg)
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_subscribe_logs_old(context: &mut GameContext, log_delegate: FFILog) -> FFIOutcome {
    let ffi_log = &mut context.logger.borrow_mut().ffi_log;
    *ffi_log = log_delegate;
    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_toggle_trace_old(context: &mut GameContext, enabled: bool) -> FFIOutcome {
    let mut logger = context.logger.borrow_mut();
    logger.trace_enabled = enabled;
    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_stellar_system(
    context: &mut GameContext,
    id: StellarSystemId,
    sun: Sun,
    parameters: StellarSystemParameters,
) -> FFIOutcome {
    context.guard(|ctx| ctx.register_stellar_system(id, sun, parameters))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_register_planet(
    context: &mut GameContext,
    id: StellarSystemId,
    planet: Planet,
) -> FFIOutcome {
    context.guard(|ctx| ctx.register_planet(id, planet))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_start_battle(
    context: &mut GameContext,
    parameters: BattleParameters,
) -> FFIOutcome {
    context.guard(|ctx| ctx.start_battle(parameters))
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_open_warp_gate(
    context: &mut GameContext,
    warp_gate: WarpGate,
    warp_gate_id: &mut i32,
) -> FFIOutcome {
    if context.battle_context.is_none() {
        return FFIOutcome::Unable;
    }

    context.guard(|ctx| {
        *warp_gate_id = ctx.battle_context
            .as_mut()
            .unwrap()
            .open_warp_gate(warp_gate);
        Ok(())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_finish_battle(context: &mut GameContext) {
    context.finish_battle();
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_update_old(
    context: &mut GameContext,
    delta_time: f32,
) -> FFIOutcome {
    if context.battle_context.is_none() {
        return FFIOutcome::Unable;
    }

    context.guard(|ctx| {
        ctx.battle_context
            .as_mut()
            .unwrap()
            .ecs
            .update(delta_time);
        Ok(())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_view_model(
    context: &mut GameContext,
) -> FFIResult<BattleStateViewModel> {
    match context.battle_context.as_ref() {
        Some(battle_ctx) => {
            let battle_state = battle_ctx.get_battle_state();
            let battle_view_state = battle_ctx.get_battle_view_state();
            let view_model = BattleStateViewModel::from(battle_state, battle_view_state);
            FFIResult::ok(view_model)
        }
        None => FFIResult::unable(),
    }
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_battle_stellar_system_view_model(
    context: &mut GameContext,
) -> FFIResult<StellarSystemViewModel> {
    match context.battle_context.as_ref() {
        Some(battle_ctx) => {
            let stellar_system = battle_ctx.get_stellar_system();

            FFIResult::ok(StellarSystemViewModel {
                id: stellar_system.id,
                sun: &stellar_system.sun,
                parameters: &stellar_system.parameters,
                planets: FFISlice::from_slice(&stellar_system.planets[..]),
            })
        }
        None => FFIResult::unable(),
    }
}

#[ffi_function]
#[ffi_surrogates(start_position = "vec3_read_ptr")]
#[no_mangle]
pub extern "C" fn ice_build_route_new(
    context: &mut GameContext,
    builder_source: RouteBuildersSource,
    start_position: &Vec3,
    start_spaceport: Spaceport,
    builder_id: &mut i32,
) -> FFIOutcome {
    context.guard(|ctx| {
        if ctx.route_builders.contains_key(&builder_source) {
            return Err("Active builder is not deleted".to_string());
        }

        ctx.route_builders_counter += 1;
        *builder_id = ctx.route_builders_counter;

        ctx.route_builders.insert(builder_source, RouteBuilder::new(
            ctx.route_builders_counter, start_position.clone(), start_spaceport),
        );

        Ok(())
    })
}

#[ffi_function]
#[ffi_surrogates(waypoint = "vec3_read_ptr")]
#[no_mangle]
pub extern "C" fn ice_build_route_add_waypoint(
    context: &mut GameContext,
    builder_source: RouteBuildersSource,
    builder_id: i32,
    waypoint: &Vec3,
) -> FFIOutcome {
    context.guard(|ctx| {
        let current = ctx.route_builders.get_mut(&builder_source);
        match current {
            None => Err(format!("Builder {builder_id} for {builder_source:?} not found").to_string()),
            Some(builder) => {
                builder.add_waypoint(waypoint.clone(), builder_id);
                Ok(())
            }
        }
    })
}

#[ffi_function]
#[ffi_surrogates(finish_position = "vec3_read_ptr")]
#[no_mangle]
pub extern "C" fn ice_spawn_spaceship(
    context: &mut GameContext,
    route_builder_source: RouteBuildersSource,
    route_builder_id: i32,
    finish_position: &Vec3,
    finish_spaceport: &Spaceport,
    owner: &Faction,
    spawner_id: i32,
) -> FFIOutcome {
    context.guard(|ctx| {
        let current = ctx.route_builders.remove(&route_builder_source);
        match current {
            None => Err(format!("Builder {route_builder_id} for {route_builder_source:?} not found").to_string()),
            Some(builder) => {
                let result = builder.build(finish_position, finish_spaceport, route_builder_id);
                match result {
                    None => Err(format!("Unable to create route {route_builder_id} for {route_builder_source:?} is none").to_string()),
                    Some(route) => {
                        let battle_ctx = ctx.battle_context.as_mut().expect("No battle context 0_o");
                        battle_ctx.spawn_spaceship(owner, spawner_id, route, SpaceshipMark::Viper); // todo: support different spaceship_info (specify on route start)
                        Ok(())
                    }
                }
            }
        }
    })
}