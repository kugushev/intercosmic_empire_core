use glam::Vec3;

#[derive(Default)]
pub struct Steering {
    pub velocity: Vec3,
}

impl Steering {
    pub fn next_seek(&mut self, position: Vec3, target: Vec3, mass: f32, max_speed: f32) -> bool {
        let desired_velocity = (target - position).normalize() * max_speed;
        if desired_velocity.is_nan() {
            return false;
        }
        let steering = Vec3::clamp_length_max(desired_velocity - self.velocity, max_speed) / mass;
        self.velocity = Vec3::clamp_length_max(self.velocity + steering, max_speed);
        true
    }
}