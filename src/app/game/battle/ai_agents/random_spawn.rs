use glam::Vec3;
use crate::app::game::battle::ai_agents::AiAgent;
use crate::app::game::battle::ai_agents::route_generation::generate_waypoints;
use crate::app::game::battle::entities::fleet::Fleet;
use crate::app::game::battle::entities::route::RouteBuildersSource;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::battle::traits::productive::Productive;
use crate::app::game::battle::traits::Spawner;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::random::Random;
use crate::trace;

pub struct RandomSpawnAgent(Random);

impl RandomSpawnAgent {
    pub fn new(seed: u64) -> Self {
        Self(Random::new(seed))
    }

    fn do_spawn(&mut self, spawner_id: i32, spawner_position: Vec3, mark: SpaceshipMark, stellar_system: &mut StellarSystem,
                my_fleet: &mut Fleet,
                logger: &LoggerRef) -> Result<(), String> {
        let faction = my_fleet.get_faction();

        let target_found = self.roll_target(stellar_system, faction, logger);
        let target = match target_found {
            Some(t) => { t }
            None => {
                // it's ok: if all planets belong to the agent, e.g. at the beginning of a battle for Red fleet
                return Ok(());
            }
        };

        let target_id = target.get_id();
        let target_position = target.get_position();

        let spawn_info = my_fleet.spawn_prepare(spawner_id,
                                                stellar_system,
                                                RouteBuildersSource::Ai,
                                                mark,
                                                my_fleet.get_faction())?;

        let route_builders = my_fleet.get_route_builders();
        let waypoints = generate_waypoints(spawner_position, [target_position].into_iter());
        for waypoint in waypoints {
            route_builders.add_waypoint(RouteBuildersSource::Ai, spawn_info.builder_id, &waypoint)?;
        }

        my_fleet.spawn_finish(spawn_info, target_id, stellar_system)
    }

    fn roll_target<'a>(&mut self, stellar_system: &'a mut StellarSystem, my_faction: Faction, _logger: &LoggerRef) -> Option<&'a dyn Spawner> {
        let mut rolled_planet: Option<&'a dyn Spawner> = None;
        let total_planets = stellar_system.planets.len();
        for planet in stellar_system.planets.iter_ref() {
            if planet.faction == my_faction {
                continue;
            }

            rolled_planet = Some(planet);

            if self.consider_as_target(total_planets) {
                break;
            }
        }

        rolled_planet
    }

    fn consider_as_target(&mut self, total_planets: u8) -> bool {
        self.0.range(0..total_planets) == 0
    }
}

impl AiAgent for RandomSpawnAgent {
    fn update(&mut self, stellar_system: &mut StellarSystem, my_fleet: &mut Fleet,
              _enemy_fleet1: Option<&Fleet>, _enemy_fleet2: Option<&Fleet>, logger: &LoggerRef) {
        let found = find_spawner(stellar_system, my_fleet, logger);
        let (spawner, mark) = match found {
            Some(t) => t,
            None => { return; }
        };

        let spawner_id = spawner.get_id();
        let spawner_position = spawner.get_position();

        if let Err(error) = self.do_spawn(spawner_id, spawner_position, mark, stellar_system, my_fleet, logger) {
            trace!(logger, format!("AI RandomSpawnOnFull Spawn Error: {error}"))
        }
    }
}


fn find_spawner<'a>(stellar_system: &'a mut StellarSystem, my_fleet: &mut Fleet, _logger: &LoggerRef)
                    -> Option<(&'a mut dyn Spawner, SpaceshipMark)> {
    let fleet_faction = my_fleet.get_faction();
    let deck = my_fleet.get_deck_mut();

    for warpgate in stellar_system.warpgates.iter_mut() {
        if warpgate.faction != fleet_faction {
            continue;
        }

        let mut spaceship = deck.peek_left().parameters();
        if warpgate.can_produce(spaceship.cost) {
            return Some((warpgate, deck.pop_left()));
        }

        spaceship = deck.peek_right().parameters();
        if warpgate.can_produce(spaceship.cost) {
            return Some((warpgate, deck.pop_right()));
        }
    }

    for planet in stellar_system.planets.iter_mut() {
        if planet.faction != fleet_faction {
            continue;
        }

        if planet.current_product < planet.get_production().max_product * 0.5 {
            continue;
        }

        let mut spaceship = deck.peek_left().parameters();
        if planet.can_produce(spaceship.cost) {
            return Some((planet, deck.pop_left()));
        }

        spaceship = deck.peek_right().parameters();
        if planet.can_produce(spaceship.cost) {
            return Some((planet, deck.pop_right()));
        }
    }

    None
}