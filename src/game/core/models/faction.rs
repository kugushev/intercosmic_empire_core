use interoptopus::ffi_type;

#[ffi_type]
#[repr(C)]
#[derive(Clone)]
pub enum Faction{
    White, // Freeman (or without a dedicated belonging)
    Red,
    Green,
    Blue,
    Grey // Old Empire Forces (can't own systems, if planet is Grey it is abandoned)
}