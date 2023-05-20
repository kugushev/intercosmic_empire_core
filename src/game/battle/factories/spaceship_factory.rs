use bevy_ecs::prelude::World;
use glam::{Quat, Vec3};
use crate::game::battle::components::spaceship::{Spaceship, SpaceshipBundle, Steering};
use crate::game::battle::components::translation::Translation;
use crate::game::battle::models::route::Route;
use crate::game::core::models::faction::Faction;
use crate::game::core::models::spaceships::spaceship_mark::SpaceshipMark;

pub fn create_spaceship(
    ecs_world: &mut World,
    route: Route,
    faction: Faction,
    model: SpaceshipMark,
) {
    let translation = Translation {
        position: route.waypoints[0],
        rotation: Quat::default(),
        scale: 0.0,
    };
    ecs_world.spawn(SpaceshipBundle {
        spaceship: Spaceship {
            route,
            faction,
            mark: model,
            target_waypoint: 0
        },
        translation,
        steering: Steering { velocity: Vec3::new(0.0, 0.0, 0.0) },
    });
}
