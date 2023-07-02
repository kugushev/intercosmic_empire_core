use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub struct BattleSettings {
    pub seed: i32
}

impl Default for BattleSettings {
    fn default() -> Self {
        Self {
            seed: 42
        }
    }
}