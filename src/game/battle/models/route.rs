use crate::game::core::models::stellar_system::spaceport::Spaceport;
use glam::Vec3;
use interoptopus::ffi_type;

pub struct Route {
    pub waypoints: Vec<Vec3>,
}

pub struct RouteBuilder {
    id: i32,
    start_position: Vec3,
    start_spaceport: Spaceport,
    raw_waypoints: Vec<Vec3>,
}

impl RouteBuilder {
    pub fn new(id: i32, start_position: Vec3, start_spaceport: Spaceport) -> RouteBuilder {
        RouteBuilder {
            id,
            start_position,
            start_spaceport,
            raw_waypoints: vec![],
        }
    }

    pub fn add_waypoint(&mut self, waypoint: Vec3) {
        if self.start_position.distance(waypoint) >= self.start_spaceport.arrival_radius {
            self.raw_waypoints.push(waypoint)
        }
    }

    pub fn build(mut self, finish_position: Vec3, finish_spaceport: Spaceport) -> Option<Route> {
        let success = self.trim_route_tail(&finish_position, &finish_spaceport);

        if success {
            Some(Route {
                waypoints: self.raw_waypoints,
            })
        } else {
            None
        }
    }

    fn trim_route_tail(&mut self, finish_position: &Vec3, finish_spaceport: &Spaceport) -> bool {
        let mut trim_count = 0;
        for raw_waypoint in self.raw_waypoints.iter().rev() {
            if finish_position.distance(*raw_waypoint) >= finish_spaceport.arrival_radius {
                break;
            }
            trim_count += 1;
        }

        let len = &self.raw_waypoints.len();
        if trim_count >= *len {
            return false;
        }

        let trim_from = len - trim_count;
        let _ = &self.raw_waypoints.drain(trim_from..);
        return true;
    }
}