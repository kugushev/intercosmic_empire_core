use interoptopus::patterns::primitives::FFIBool;
use crate::app::game::battle::ai_agents::AiAgent;
use crate::app::game::battle::entities::fleet::Fleet;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::core::battle_settings::BattleSettings;
use crate::app::game::core::faction::Faction;
use crate::app::utils::delta_time::DeltaTime;
use crate::app::utils::interop_logger::LoggerRef;
use crate::app::utils::struct_vec::StructVec5;
use crate::trace;

pub struct FleetSet {
    player_fleet: Option<Fleet>,
    enemy_fleet: Option<(Fleet, Box<dyn AiAgent>)>,
    ally_fleet: Option<(Fleet, Box<dyn AiAgent>)>,
}

impl FleetSet {
    pub fn new(settings: &BattleSettings, logger: &LoggerRef) -> Self {
        let player_fleet = if settings.player_fleet_enabled == FFIBool::TRUE {
            trace!(logger, "Green fleet added");
            Some(Fleet::new(Faction::Green))
        } else { None };

        let enemy_fleet: Option<(Fleet, Box<dyn AiAgent>)> = if settings.enemy_fleet_enabled == FFIBool::TRUE {
            trace!(logger, "Red fleet added");
            Some((
                Fleet::new(Faction::Red),
                settings.enemy_fleet_ai.create_agent(settings.seed)
            ))
        } else { None };

        let ally_fleet: Option<(Fleet, Box<dyn AiAgent>)> = if settings.ally_fleet_enabled == FFIBool::TRUE {
            trace!(logger, "Blue fleet added");
            Some((
                Fleet::new(Faction::Blue),
                settings.ally_fleet_ai.create_agent(settings.seed)
            ))
        } else { None };

        Self { player_fleet, enemy_fleet, ally_fleet }
    }

    pub fn update(&mut self, stellar_system: &mut StellarSystem, delta: DeltaTime, logger: &LoggerRef) {
        if let Some(fleet) = &mut self.player_fleet {
            fleet.update(stellar_system, delta, logger)
        }
        if let Some((fleet, agent)) = &mut self.enemy_fleet {
            agent.update(stellar_system,
                         fleet,
                         self.player_fleet.as_ref(),
                         self.ally_fleet.as_ref().map(|t| &t.0), logger);
            fleet.update(stellar_system, delta, logger)
        }
        if let Some((fleet, agent)) = &mut self.ally_fleet {
            agent.update(stellar_system,
                         fleet,
                         self.enemy_fleet.as_ref().map(|t| &t.0),
                         None,
                         logger);
            fleet.update(stellar_system, delta, logger)
        }
    }

    pub fn get_fleets_factions(&self) -> Result<StructVec5<Faction>, String> {
        let mut fleets = StructVec5::default();
        if self.player_fleet.is_some() {
            fleets.add(Faction::Green)?;
        }
        if self.enemy_fleet.is_some() {
            fleets.add(Faction::Red)?;
        }
        if self.ally_fleet.is_some() {
            fleets.add(Faction::Blue)?;
        }
        Ok(fleets)
    }

    pub fn get_fleet_mut(&mut self, faction: Faction) -> Result<&mut Fleet, String> {
        match faction {
            Faction::Green => { self.player_fleet.as_mut().ok_or("Green fleet not found".to_string()) }
            Faction::Red => { self.enemy_fleet.as_mut().map(|p| &mut p.0).ok_or("Red fleet not found".to_string()) }
            Faction::Blue => { self.ally_fleet.as_mut().map(|p| &mut p.0).ok_or("Blue fleet not found".to_string()) }
            _ => { Err(format!("Unexpected faction {faction:?}").to_string()) }
        }
    }

    pub fn get_fleet_ref(&self, faction: Faction) -> Result<&Fleet, String> {
        match faction {
            Faction::Green => { self.player_fleet.as_ref().ok_or("Green fleet not found".to_string()) }
            Faction::Red => { self.enemy_fleet.as_ref().map(|p| &p.0).ok_or("Red fleet not found".to_string()) }
            Faction::Blue => { self.ally_fleet.as_ref().map(|p| &p.0).ok_or("Blue fleet not found".to_string()) }
            _ => { Err(format!("Unexpected faction {faction:?}").to_string()) }
        }
    }
}