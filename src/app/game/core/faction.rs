use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Faction{
    White,  // Neutral, background ships, e.g. refugees
    Red,    // Enemy
    Green,  // Player
    Blue,   // Ally
    Grey    // Abandoned, berserk ships that attacks everything
}

pub const PLAYER_FACTION: Faction = Faction::Green;