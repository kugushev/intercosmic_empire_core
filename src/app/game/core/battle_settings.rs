use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct BattleSettings {
    pub seed: u64,
    pub day_of_year: u16
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            seed: 42,
            day_of_year: 0
        }
    }
}