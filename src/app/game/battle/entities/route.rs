use std::collections::HashMap;
use derive_getters::Getters;
use glam::Vec3;
use interoptopus::ffi_type;
use crate::app::game::core::stellar_system::spaceport::Spaceport;

#[derive(Getters)]
pub struct Route {
    waypoints: Vec<Vec3>,
    start_position: Vec3,
    start_spaceport: Spaceport,
    finish_position: Vec3,
    finish_spaceport: Spaceport,
    finish_id: i32,
}

impl Route {
    pub fn new(waypoints: Vec<Vec3>, start_position: Vec3, start_spaceport: Spaceport, finish_position: Vec3, finish_spaceport: Spaceport, finish_id: i32) -> Self {
        assert!(waypoints.len() > 0, "Waypoints shouldn't be empty");
        Self { waypoints, start_position, start_spaceport, finish_position, finish_spaceport, finish_id }
    }
}

pub struct RouteBuilder {
    id: i32,
    start_position: Vec3,
    start_spaceport: Spaceport,
    raw_waypoints: Vec<Vec3>,
}

#[derive(Default)]
pub struct RouteBuilders {
    slots: HashMap<RouteBuildersSource, RouteBuilder>,
    counter: i32,
}

impl RouteBuilders {
    pub fn drop_current(&mut self, builder_source: RouteBuildersSource) {
        self.slots.remove(&builder_source);
    }

    pub fn new_builders(&mut self, builder_source: RouteBuildersSource, start_position: Vec3,
                        start_spaceport: Spaceport) -> Result<i32, String> {
        if self.slots.contains_key(&builder_source) {
            return Err(format!("Builder {builder_source:?} is still active"));
        }

        self.counter += 1;

        self.slots.insert(builder_source, RouteBuilder::new(
            self.counter, start_position, start_spaceport),
        );

        Ok(self.counter)
    }

    pub fn add_waypoint(&mut self, builder_source: RouteBuildersSource, builder_id: i32, waypoint: &Vec3) -> Result<(), String> {
        let current = self.slots.get_mut(&builder_source);
        match current {
            Some(builder) => {
                builder.add_waypoint(waypoint.clone(), builder_id)
            }
            None => Err(format!("Builder {builder_id} for {builder_source:?} not found").to_string()),
        }
    }

    pub fn finish(&mut self, builder_source: RouteBuildersSource, builder_id: i32,
                  finish_position: Vec3, finish_spaceport: Spaceport,
                  finish_id: i32) -> Result<Route, String> {
        let current = self.slots.remove(&builder_source);
        match current {
            None => Err(format!("Builder {builder_id} for {builder_source:?} not found").to_string()),
            Some(builder) => {
                builder.build(finish_id, finish_position, finish_spaceport, builder_id)
            }
        }
    }
}

#[ffi_type]
#[repr(C)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum RouteBuildersSource {
    LeftHand,
    RightHand,
    Ai,
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

    pub fn add_waypoint(&mut self, waypoint: Vec3, builder_id: i32) -> Result<(), String> {
        if self.id != builder_id {
            return Err(format!("Unexpected builder id {builder_id}, expected {}", self.id));
        }

        if self.start_position.distance(waypoint) >= *self.start_spaceport.orbit_radius() {
            self.raw_waypoints.push(waypoint)
        }
        Ok(())
    }

    pub fn build(mut self, finish_id: i32, finish_position: Vec3, finish_spaceport: Spaceport, builder_id: i32) -> Result<Route, String> {
        if self.id != builder_id {
            return Err(format!("Unexpected builder id {builder_id}, expected {}", self.id));
        }

        self.trim_route_tail(finish_position, finish_spaceport.clone());

        if self.raw_waypoints.is_empty() {
            return Err("Empty waypoints".to_string());
        }


        Ok(Route::new(self.raw_waypoints, self.start_position, self.start_spaceport,
                      finish_position, finish_spaceport, finish_id))
    }

    fn trim_route_tail(&mut self, finish_position: Vec3, finish_spaceport: Spaceport) {
        let mut trim_count = 0;
        for raw_waypoint in self.raw_waypoints.iter().rev() {
            if finish_position.distance(*raw_waypoint) >= *finish_spaceport.orbit_radius() {
                break;
            }
            trim_count += 1;
        }

        let len = &self.raw_waypoints.len();
        if trim_count >= *len {
            return;
        }

        let trim_from = len - trim_count;
        let _ = &self.raw_waypoints.drain(trim_from..);
    }
}