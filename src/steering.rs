use glam::Vec3;

pub(crate) fn seek(position: &Vec3, target: &Vec3, mass: f32, max_speed: f32,
                   current_velocity: &Vec3) -> Vec3 {
    let desired_velocity = (*target - *position).normalize() * max_speed;

    let steering = Vec3::clamp_length_max(desired_velocity - *current_velocity, max_speed) / mass;
    return Vec3::clamp_length_max(*current_velocity + steering, max_speed);
}