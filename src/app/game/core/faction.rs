use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Faction {
    Neutral,
    Enemy,
    Player,
    Ally,
    Abandoned,
}