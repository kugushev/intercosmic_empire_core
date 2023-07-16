use glam::Vec3;
use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::behavior_arrival::BehaviorArrival;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::battle::services::steering_behavior::Steering;
use crate::app::game::core::spaceship_info::spaceship_parameters::SpaceshipParameters;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::quat_extra::{QuatEx, VEC3_UP};
use crate::trace;

#[derive(Default)]
pub(super) struct BehaviorMove {
    steering: Steering,
}

pub const SUFFICIENT_DISTANCE_TO_TARGET: f32 = 0.025;


impl BehaviorMove {
    pub fn seek_to_waypoint(&mut self, shared_state: &mut SharedState, waypoint: Vec3,
                            spaceship_parameters: &SpaceshipParameters, delta: DeltaTime,
                            logger: &LoggerRef) {
        self.steering.next_seek(shared_state.translation.position, waypoint, spaceship_parameters.mass,
                                spaceship_parameters.max_speed);

        shared_state.translation.position += self.steering.velocity * delta.seconds();

        trace!(logger, format!("Seek v: {}", self.steering.velocity));

        if self.steering.velocity.length() != 0.0 {
            let quat = self.steering.velocity.look_rotation(VEC3_UP);
            trace!(logger, format!("New quat: {}", quat));
            shared_state.translation.rotation = quat;
            trace!(logger, format!("Set quat: {}", shared_state.translation.rotation));
        }
    }
}

impl Behavior for BehaviorMove {
    fn update(&mut self, shared_state: &mut SharedState, _stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        let all_waypoints_visited = shared_state.target_waypoint >= shared_state.route.waypoints().len();
        if all_waypoints_visited {
            trace!(logger, "Move to Arrival");
            return Some(Box::new(BehaviorArrival::new(shared_state)));
        }

        let target = shared_state.route.waypoints()[shared_state.target_waypoint];
        let parameters = SpaceshipParameters::get(shared_state.mark);
        self.seek_to_waypoint(shared_state, target, parameters, delta, logger);

        trace!(logger, format!("Move to {target} p:{} r:{}", shared_state.translation.position, shared_state.translation.rotation));

        if shared_state.translation.position.distance(target) <= SUFFICIENT_DISTANCE_TO_TARGET {
            shared_state.target_waypoint += 1;
        }
        None
    }
}

