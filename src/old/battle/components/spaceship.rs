use bevy_ecs::prelude::{Component, Bundle};
use glam::Vec3;

use crate::old::{core::models::{spaceships::spaceship_mark::SpaceshipMark, faction::Faction}, battle::models::route::Route};

use super::translation::Translation;

#[derive(Component)]
pub struct Spaceship
{
    pub route: Route,
    pub target_waypoint: usize,
    pub faction: Faction,
    pub mark: SpaceshipMark,
}

#[derive(Bundle)]
pub struct SpaceshipBundle{
    pub spaceship: Spaceship,
    pub translation: Translation,
    pub steering:Steering
}

#[derive(Component)]
pub struct Steering {
    pub velocity: Vec3
}