use glam::Vec3;

pub(crate) fn seek(position: Vec3, target: Vec3, mass: f32, maxSpeed: f32,
                   currentVelocity: Vec3) -> Vec3 {
    let desiredVelocity = (target - position).normalize() * maxSpeed;

    let steering = Vec3::clamp_length_max(desiredVelocity - currentVelocity, maxSpeed) / mass;
    return Vec3::clamp_length_max(currentVelocity + steering, maxSpeed);
}