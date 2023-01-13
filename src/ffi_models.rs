use interoptopus::ffi_type;
use interoptopus::patterns::slice::FFISlice;
use crate::game::battle::models::warp_gate::WarpGate;
use crate::game::core::models::stellar_system::{Planet, StellarSystemId, StellarSystemParameters, Sun};

#[ffi_type]
#[repr(C)]
pub enum FFIResult {
    Ok,
    NullPointerError,
    NotNullPointerError,
}

#[ffi_type]
#[repr(C)]
pub struct StellarSystemViewModel<'a> {
    pub id: StellarSystemId,
    pub sun: &'a Sun,
    pub parameters: &'a StellarSystemParameters,
    pub planets: FFISlice<'a, Planet>,
    pub warp_gates: FFISlice<'a, WarpGate>,
}