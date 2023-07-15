use glam::{Quat, Vec3};
use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::behavior_disposed::BehaviorDisposed;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::quat_extra::{QuatEx, VEC3_DOWN, VEC3_FORWARD};
use crate::trace;

pub(super) struct BehaviorArrival {
    orbit_position: Vec3,
    orbit_rotation: Quat,
    land_position: Vec3,
    land_rotation: Quat,
    arrival_time: f32,
}

const MAX_ARRIVAL_TIME_SECONDS: f32 = 3.0;

impl BehaviorArrival {
    pub(super) fn new(shared_state: &SharedState) -> Self {
        let orbit_position = shared_state.translation.position;
        let orbit_rotation = shared_state.translation.rotation;
        let orbit_to_centre = *shared_state.route.finish_position() - orbit_position;
        let orbit_to_land = orbit_to_centre.normalize() *
            (orbit_to_centre.length() - *shared_state.route.finish_spaceport().surface_radius()).max(0.0);
        let land_position = orbit_position + orbit_to_land;
        let tangent = orbit_to_centre.cross(if orbit_to_centre != VEC3_DOWN { VEC3_DOWN } else { VEC3_FORWARD });
        let land_rotation = tangent.look_rotation(-orbit_to_land);
        Self { orbit_position, orbit_rotation, land_position, land_rotation, arrival_time: 0.0 }
    }
}

impl Behavior for BehaviorArrival {
    fn update(&mut self, shared_state: &mut SharedState, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        if self.arrival_time > MAX_ARRIVAL_TIME_SECONDS {
            trace!(logger, "Arrival to Disposed");
            return Some(Box::new(BehaviorDisposed));
        }

        let t = self.arrival_time / MAX_ARRIVAL_TIME_SECONDS;
        shared_state.translation.position = self.orbit_position.lerp(self.land_position, t);
        shared_state.translation.rotation = self.orbit_rotation.lerp(self.land_rotation, t);
        shared_state.translation.scale = 1.0 - t;
        self.arrival_time += delta.seconds();

        trace!(logger, format!("Arrival p:{} r:{} s:{} t:{}", shared_state.translation.position, shared_state.translation.rotation, shared_state.translation.scale, self.arrival_time));

        None
    }
}