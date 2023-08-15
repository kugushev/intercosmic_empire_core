use crate::app::game::battle::entities::fleet::Fleet;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::utils::interop_logger::LoggerRef;
use interoptopus::ffi_type;
use crate::app::game::battle::ai_agents::random_spawn::RandomSpawnAgent;
use crate::app::utils::delta_time::DeltaTime;

pub mod random_spawn;
pub mod route_generation;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone)]
pub enum AiAgentType {
    NoOp,
    RandomSpawn,
}

impl AiAgentType {
    pub fn create_agent(&self, seed: u64) -> Box<dyn AiAgent> {
        match self {
            AiAgentType::NoOp => { Box::new(NoOpAgent) }
            AiAgentType::RandomSpawn => { Box::new(RandomSpawnAgent::new(seed)) }
        }
    }
}

pub trait AiAgent {
    fn update(&mut self, stellar_system: &mut StellarSystem, delta: DeltaTime, my_fleet: &mut Fleet,
              enemy_fleet1: Option<&Fleet>, enemy_fleet2: Option<&Fleet>, logger: &LoggerRef);
}

pub struct NoOpAgent;

impl AiAgent for NoOpAgent {
    fn update(&mut self, _stellar_system: &mut StellarSystem, _delta: DeltaTime, _my_fleet: &mut Fleet, _enemy_fleet1: Option<&Fleet>, _enemy_fleet2: Option<&Fleet>, _logger: &LoggerRef) {}
}