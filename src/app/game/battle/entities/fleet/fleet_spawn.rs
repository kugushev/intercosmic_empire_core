use glam::Vec3;
use interoptopus::{ffi_type, ffi_function, ffi_surrogates};
use crate::ffi::surrogates::vec3_read_ptr;
use crate::app::AppContext;
use crate::app::game::battle::current_battle_exec;
use crate::app::game::battle::entities::fleet::fleet_exec;
use crate::app::game::battle::entities::route::{RouteBuilders, RouteBuildersSource};
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::ffi::utils::{FFIOutcome, FFIResult};

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_spawn_begin(
    context: &mut AppContext,
    faction: Faction,
    builder_source: RouteBuildersSource,
    spawner_id: i32,
    mark: SpaceshipMark,
) -> FFIResult<SpawnInfo> {
    current_battle_exec(context, |b| {
        let stellar_system = &mut b.stellar_system;
        let fleet = b.fleets.get_fleet_mut(faction)?;
        fleet.spawn_prepare(spawner_id, stellar_system, builder_source, mark, faction)
    })
}

#[ffi_function]
#[ffi_surrogates(waypoint = "vec3_read_ptr")]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_spawn_add_waypoint(
    context: &mut AppContext,
    info: SpawnInfo,
    waypoint: &Vec3,
) -> FFIOutcome {
    route_builder_exec(context, info.faction, |builders| {
        builders.add_waypoint(info.builder_source, info.builder_id, waypoint)
    }).outcome
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_spawn_finish(
    context: &mut AppContext,
    info: SpawnInfo,
    finish_id: i32,
) -> FFIOutcome {
    current_battle_exec(context, |b| {
        let stellar_system = &mut b.stellar_system;
        let fleet = b.fleets.get_fleet_mut(info.faction)?;
        fleet.spawn_finish(info, finish_id, stellar_system)
    }).outcome
}

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpawnInfo {
    pub faction: Faction,
    pub builder_source: RouteBuildersSource,
    pub spawner_id: i32,
    pub builder_id: i32,
    pub spaceship_mark: SpaceshipMark,
}

fn route_builder_exec<T>(context: &mut AppContext, faction: Faction, action: impl FnOnce(&mut RouteBuilders) -> Result<T, String>) -> FFIResult<T> {
    fleet_exec(context, faction, |fleet| {
        let builders = &mut fleet.route_builders;
        action(builders)
    })
}