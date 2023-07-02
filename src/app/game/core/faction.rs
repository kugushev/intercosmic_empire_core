use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Faction{
    White, // Freeman (or without a dedicated belonging)
    Red,
    Green,
    Blue,
    Grey // if a planet is Grey it is abandoned
}

pub const PLAYER_FACTION: Faction = Faction::Green;