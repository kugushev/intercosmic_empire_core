use interoptopus::ffi_function;
use crate::app::AppContext;
use crate::app::game::battle::current_battle_exec;
use crate::app::game::core::faction::Faction;
use crate::app::game::core::spaceship_info::SpaceshipMark;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_deck_peek_left(
    context: &mut AppContext,
    faction: Faction
) -> FFIResult<SpaceshipMark> {
    current_battle_exec(context, |b| {
        let fleet = b.fleets.get_fleet_mut(faction)?;
        Ok(fleet.deck.peek_left())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_deck_peek_right(
    context: &mut AppContext,
    faction: Faction
) -> FFIResult<SpaceshipMark> {
    current_battle_exec(context, |b| {
        let fleet = b.fleets.get_fleet_mut(faction)?;
        Ok(fleet.deck.peek_right())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_deck_pop_left(
    context: &mut AppContext,
    faction: Faction
) -> FFIResult<SpaceshipMark> {
    current_battle_exec(context, |b| {
        let fleet = b.fleets.get_fleet_mut(faction)?;
        Ok(fleet.deck.pop_left())
    })
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_battle_fleet_deck_pop_right(
    context: &mut AppContext,
    faction: Faction
) -> FFIResult<SpaceshipMark> {
    current_battle_exec(context, |b| {
        let fleet = b.fleets.get_fleet_mut(faction)?;
        Ok(fleet.deck.pop_right())
    })
}

pub struct Deck;

impl Deck {
    pub fn peek_left(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }

    pub fn peek_right(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }

    pub fn pop_left(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }

    pub fn pop_right(&self) -> SpaceshipMark {
        SpaceshipMark::Viper
    }
}