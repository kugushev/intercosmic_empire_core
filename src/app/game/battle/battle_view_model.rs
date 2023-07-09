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
    pub fn new(battle: &'a Battle) -> Self {
        let mut fleets = StructVec5::default();
        for faction in battle.fleets.keys() {
            if let Err(error) = fleets.add(*faction) {
                panic!("Can't fill fleet in view model {}", error)
            }
        }

        Self {
            stellar_system: &battle.stellar_system,
            fleets,
        }
    }
}