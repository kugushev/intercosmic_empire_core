use glam::{Quat, Vec3};
use crate::app::game::battle::entities::spaceship::behavior::Behavior;
use crate::app::game::battle::entities::spaceship::behavior_move::BehaviorMove;
use crate::app::game::battle::entities::spaceship::shared_state::SharedState;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::quat_extra::{QuatEx, VEC3_DOWN, VEC3_FORWARD, VEC3_UP};
use crate::trace;

pub(super) struct BehaviorDeparture {
    orbit_position: Vec3,
    orbit_rotation: Quat,
    land_position: Vec3,
    land_rotation: Quat,
    departure_time: f32,
}

const MAX_DEPARTURE_TIME_SECONDS: f32 = 3.0;


impl BehaviorDeparture {
    pub(super) fn new(shared_state: &SharedState) -> Self {
        let route = &shared_state.route;
        let orbit_position = *route.waypoints().first().expect("Waypoints shouldn't be empty");
        let orbit_to_centre = *route.start_position() - orbit_position;
        let orbit_rotation = (-orbit_to_centre).look_rotation(VEC3_UP);
        let orbit_to_land = orbit_to_centre.normalize() *
            (orbit_to_centre.length() - *route.start_spaceport().surface_radius()).max(0.0);
        let land_position = orbit_position + orbit_to_land;
        let tangent = orbit_to_centre.cross(if orbit_to_centre != VEC3_DOWN { VEC3_DOWN } else { VEC3_FORWARD });
        let land_rotation = tangent.look_rotation(-orbit_to_land);
        Self { orbit_position, orbit_rotation, land_position, land_rotation, departure_time: 0.0 }
    }
}

impl Behavior for BehaviorDeparture {
    fn update(&mut self, shared_state: &mut SharedState, _stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) -> Option<Box<dyn Behavior>> {
        if self.departure_time > MAX_DEPARTURE_TIME_SECONDS {
            trace!(logger, "Departure to Move");
            return Some(Box::<BehaviorMove>::default());
        }

        let t = self.departure_time / MAX_DEPARTURE_TIME_SECONDS;
        shared_state.translation.position = self.land_position.lerp(self.orbit_position, t);
        shared_state.translation.rotation = self.land_rotation.lerp(self.orbit_rotation, t);
        shared_state.translation.scale = t;
        self.departure_time += delta.seconds();

        trace!(logger, format!("Departure p:{} r:{} s:{} t:{}", shared_state.translation.position, shared_state.translation.rotation, shared_state.translation.scale, self.departure_time));
        None
    }
}