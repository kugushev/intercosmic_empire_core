use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct BattleSettings {
    pub seed: u64,
    pub day_of_year: u16,
    pub player_fleet_enabled: bool,
    pub enemy_fleet_enabled: bool,
    pub ally_fleet_enabled: bool,
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            seed: 42,
            day_of_year: 0,
            player_fleet_enabled: false,
            enemy_fleet_enabled: true,
            ally_fleet_enabled: true
        }
    }
}