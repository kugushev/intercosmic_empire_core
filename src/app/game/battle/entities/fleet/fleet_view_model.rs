use interoptopus::{ffi_type, ffi_function, ffi_surrogates};
use interoptopus::patterns::slice::FFISlice;
use crate::app::AppContext;
use crate::app::game::battle::entities::fleet::{Fleet, fleet_ref};
use crate::app::game::battle::entities::spaceship::SpaceshipViewModel;
use crate::app::game::core::faction::Faction;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_get_vm(
    context: &mut AppContext,
    faction: Faction,
) -> FFIResult<FleetViewModel> {
    let game = &mut context.game;
    let guard = &mut context.guard;

    guard.wrap(|| {
        let fleet = fleet_ref(game, faction)?;
        Ok(fleet.into())
    })
}

#[ffi_type]
#[ffi_surrogates(position = "vec3")]
#[repr(C)]
pub struct FleetViewModel<'a> {
    pub spaceships: FFISlice<'a, SpaceshipViewModel>,
}

impl<'a> From<&'a Fleet> for FleetViewModel<'a> {
    fn from(value: &'a Fleet) -> Self {
        Self {
            spaceships: FFISlice::from_slice(&value.spaceships_vm)
        }
    }
}