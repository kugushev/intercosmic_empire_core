use interoptopus::ffi_type;
use crate::app::game::battle::Battle;
use crate::app::game::battle::entities::stellar_system::StellarSystem;
use crate::app::game::core::faction::Faction;
use crate::app::utils::struct_vec::StructVec5;

#[ffi_type]
#[repr(C)]
pub struct BattleViewModel<'a> {
    pub stellar_system: &'a StellarSystem,
    pub fleets: StructVec5<Faction>,
}

impl<'a> BattleViewModel<'a> {
    pub fn new(battle: &'a Battle) -> Result<Self, String> {
        let fleets = battle.fleets.get_fleets_factions()?;
        Ok(Self {
            stellar_system: &battle.stellar_system,
            fleets,
        })
    }
}