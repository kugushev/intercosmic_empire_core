use bevy_ecs::prelude::{Component, Bundle};

use crate::game::{core::models::{spaceships::spaceship_model::SpaceshipModel, faction::Faction}, battle::models::route::Route};

use super::translation::Translation;

#[derive(Component)]
pub struct Spaceship
{
    route: Route,
    faction: Faction,
    model: SpaceshipModel,
}

#[derive(Bundle)]
pub struct SpaceshipBundle{
    spaceship: Spaceship,
    translation: Translation,
}