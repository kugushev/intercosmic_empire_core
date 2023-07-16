use glam::Quat;
use crate::app::game::battle::entities::route::Route;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::spaceship_parameters::SpaceshipParameters;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::app::utils::translation::Translation;

pub(super) struct SharedState {
    pub route: Route,
    pub faction: Faction,
    pub mark: SpaceshipMark,
    pub translation: Translation,
    pub target_waypoint: usize,
    pub product: f32
}

impl SharedState {
    pub fn new(mark: SpaceshipMark, route: Route, faction: Faction) -> Self {
        Self {
            translation: Translation {
                position: *route.start_position(),
                rotation: Quat::default(),
                scale: 1.0,
            },
            route,
            target_waypoint: 0,
            faction,
            mark,
            product: SpaceshipParameters::get(mark).cost as f32
        }
    }
}