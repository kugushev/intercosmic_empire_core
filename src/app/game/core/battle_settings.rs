use interoptopus::ffi_type;
use interoptopus::patterns::primitives::FFIBool;
use crate::app::game::battle::ai_agents::AiAgentType;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct BattleSettings {
    pub seed: u64,
    pub day_of_year: u16,
    pub player_fleet_enabled: FFIBool,
    pub enemy_fleet_enabled: FFIBool,
    pub enemy_fleet_ai: AiAgentType,
    pub ally_fleet_enabled: FFIBool,
    pub ally_fleet_ai: AiAgentType,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            seed: 42,
            day_of_year: 0,
            player_fleet_enabled: FFIBool::FALSE,
            enemy_fleet_enabled: FFIBool::TRUE,
            enemy_fleet_ai: AiAgentType::RandomSpawn,
            ally_fleet_enabled: FFIBool::TRUE,
            ally_fleet_ai: AiAgentType::RandomSpawn
        }
    }
}