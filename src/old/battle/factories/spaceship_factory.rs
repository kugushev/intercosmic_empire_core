use bevy_ecs::prelude::World;
use glam::{Quat, Vec3};
use crate::old::battle::components::spaceship::{Spaceship, SpaceshipBundle, Steering};
use crate::old::battle::components::translation::Translation;
use crate::old::battle::models::route::Route;
use crate::old::core::models::faction::Faction;
use crate::old::core::models::spaceships::spaceship_mark::SpaceshipMark;

pub fn create_spaceship(
    ecs_world: &mut World,
    route: Route,
    faction: Faction,
    mark: SpaceshipMark,
) {
    let translation = Translation {
        position: route.waypoints[0],
        rotation: Quat::default(),
        scale: 1.0,
    };
    ecs_world.spawn(SpaceshipBundle {
        spaceship: Spaceship {
            route,
            faction,
            mark,
            target_waypoint: 0
        },
        translation,
        steering: Steering { velocity: Vec3::new(0.0, 0.0, 0.0) },
    });
}
