use bevy_ecs::prelude::{Mut, NonSend, Query, Res};
use glam::{Quat, Vec3};
use crate::old::battle::components::spaceship::{Spaceship, Steering};
use crate::old::battle::components::translation::Translation;
use crate::old::battle::utils::game_time::GameTime;
use crate::old::core::models::spaceships::spaceship_parameters::SpaceshipParameters;
use crate::old::utils::interop_logger::LoggerRef;
use crate::trace;

pub const SUFFICIENT_DISTANCE_TO_TARGET: f32 = 0.025;

pub(crate) fn spaceship_move(mut query: Query<(&mut Spaceship, &mut Translation, &mut Steering)>,
                             time: Res<GameTime>, logger: NonSend<LoggerRef>) {
    for (mut spaceship, mut translation, mut steering) in &mut query {

        // todo: spaceshipViewRef.View.Weapon.Extinguish();

        let all_waypoints_visited = spaceship.target_waypoint >= spaceship.route.waypoints.len();
        if !all_waypoints_visited {
            let target = &spaceship.route.waypoints[spaceship.target_waypoint];
            let parameters = SpaceshipParameters::get_parameters(&spaceship.mark);
            seek_to_waypoint(&mut translation, *target, parameters, &mut steering, time.delta_time);

            trace!(logger, format!("Move to {target} p:{} r:{}", translation.position, translation.rotation));

            if translation.position.distance(*target) <= SUFFICIENT_DISTANCE_TO_TARGET {
                spaceship.target_waypoint += 1;
            }
        } else {
            trace!(logger, format!("All waypoints are visited"));
        }
    }
}

fn seek_to_waypoint(translation: &mut Mut<Translation>, waypoint: Vec3,
                    spaceship_parameters: &SpaceshipParameters, steering: &mut Mut<Steering>,
                    delta_time: f32) {
    steering.velocity = seek(&translation.position, &waypoint, spaceship_parameters.mass,
                             spaceship_parameters.max_speed, &steering.velocity);

    translation.position += steering.velocity * delta_time;

    if steering.velocity.length() != 0.0 {
        translation.rotation = Quat::from_axis_angle(steering.velocity, 0.0);
    }
}

fn seek(position: &Vec3, target: &Vec3, mass: f32, max_speed: f32,
        current_velocity: &Vec3) -> Vec3 {
    let desired_velocity = (*target - *position).normalize() * max_speed;
    if desired_velocity.is_nan() {
        return *current_velocity;
    }
    let steering = Vec3::clamp_length_max(desired_velocity - *current_velocity, max_speed) / mass;
    Vec3::clamp_length_max(*current_velocity + steering, max_speed)
}