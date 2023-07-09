use glam::{Quat, Vec3};
use interoptopus::{ffi_surrogates, ffi_type};
use crate::app::game::battle::entities::route::Route;
use crate::app::game::battle::services::steering_behavior::Steering;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::spaceship_parameters::SpaceshipParameters;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::translation::{FFITranslation, Translation};
use crate::app::utils::zero;
use crate::trace;

pub const EMPTY_SPACESHIP_VM: u64 = 0;

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct SpaceshipViewModel {
    pub id: u64,
    // todo: if Unity->View has not the same ID, reset view
    pub translation: FFITranslation,
    pub faction: Faction,
    pub mark: SpaceshipMark,
}

impl SpaceshipViewModel {
    pub fn empty() -> Self {
        Self {
            id: EMPTY_SPACESHIP_VM,
            translation: zero(),
            faction: zero(),
            mark: zero(),
        }
    }
}

pub struct Spaceship {
    id: u64,
    translation: Translation,
    route: Route,
    target_waypoint: usize,
    faction: Faction,
    mark: SpaceshipMark,
    state: State,
    steering: Steering,
}

impl Spaceship {
    pub fn new(id: u64, mark: SpaceshipMark, route: Route, faction: Faction) -> Self {
        Self {
            id,
            translation: Translation {
                position: route.start_position,
                rotation: Quat::default(),
                scale: 1.0,
            },
            route,
            target_waypoint: 0,
            faction,
            mark,
            state: State::Departure,
            steering: Steering::default(),
        }
    }

    pub fn get_vm(&self) -> SpaceshipViewModel {
        SpaceshipViewModel {
            id: self.id,
            translation: (&self.translation).into(),
            faction: self.faction,
            mark: self.mark,
        }
    }

    pub fn update(&mut self, delta: DeltaTime, logger: &LoggerRef) {
        match self.state {
            State::Departure => {
                // todo: implement departure animation
                self.state = State::Move;
            }
            State::Move => {
                self.do_move(delta, logger);
            }
            State::Disposed => {
                todo!()
            }
        }
    }

    fn do_move(&mut self, delta: DeltaTime, logger: &LoggerRef) {
        // todo: spaceshipViewRef.View.Weapon.Extinguish();

        let all_waypoints_visited = self.target_waypoint >= self.route.waypoints.len();
        if !all_waypoints_visited {
            let target = self.route.waypoints[self.target_waypoint];
            let parameters = SpaceshipParameters::get(self.mark);
            self.seek_to_waypoint(target, parameters, delta);

            trace!(logger, format!("Move to {target} p:{} r:{}", self.translation.position, self.translation.rotation));

            if self.translation.position.distance(target) <= SUFFICIENT_DISTANCE_TO_TARGET {
                self.target_waypoint += 1;
            }
        } else {
            trace!(logger, format!("All waypoints are visited"));
        }
    }

    fn seek_to_waypoint(&mut self, waypoint: Vec3, spaceship_parameters: &SpaceshipParameters, delta: DeltaTime) {
        self.steering.next_seek(self.translation.position, waypoint, spaceship_parameters.mass,
                                spaceship_parameters.max_speed);

        self.translation.position += self.steering.velocity * delta.seconds();

        if self.steering.velocity.length() != 0.0 {
            self.translation.rotation = Quat::from_axis_angle(self.steering.velocity, 0.0);
        }
    }

    pub fn disposed(&self) -> bool { matches!(self.state, State::Disposed) }
}

pub const SUFFICIENT_DISTANCE_TO_TARGET: f32 = 0.025;

enum State {
    Departure,
    Move,
    Disposed,
}