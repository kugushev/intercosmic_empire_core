use glam::Vec3;
use crate::obstacle::Obstacle;

pub(crate) fn seek(position: Vec3, target: Vec3, mass: f32, max_speed: f32, current_velocity: Vec3) -> Vec3 {
    let desiredVelocity = (target - position).normalize() * max_speed;

    let steering = Vec3::clamp_length_max(desiredVelocity - current_velocity, max_speed) / mass;
    return Vec3::clamp_length_max(current_velocity + steering, max_speed);
}

pub(crate) fn avoid(obstacles: &[Obstacle], position: Vec3, mass: f32, max_speed: f32, delta_time: f32, current_velocity: Vec3) -> Vec3 {
    let ahead = position + current_velocity * delta_time;
    for obstacle in obstacles {
        let position = obstacle.position.map();
        if position.distance(ahead) < obstacle.radius {
            let steering = (ahead - position).normalize() * max_speed / mass;
            return Vec3::clamp_length_max(current_velocity + steering, max_speed);
        }
    }
    return current_velocity;
}