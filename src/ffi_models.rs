use std::mem::MaybeUninit;
use interoptopus::ffi_type;
use interoptopus::patterns::slice::FFISlice;
use crate::game::battle::models::warp_gate::WarpGate;
use crate::game::core::models::stellar_system::{Planet, StellarSystemId, StellarSystemParameters, Sun};
use crate::game::battle::models::battle_view_model::BattleViewModel;

#[ffi_type]
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum FFIOutcome {
    Ok,
    Unable,
    Error,
}

#[ffi_type]
#[repr(C)]
pub struct FFIResult<T> {
    pub value: T,
    pub outcome: FFIOutcome,
}

impl<T> FFIResult<T> {
    pub fn ok(value: T) -> FFIResult<T> {
        FFIResult {
            value,
            outcome: FFIOutcome::Ok,
        }
    }

    pub fn unable() -> FFIResult<T> {
        FFIResult {
            value: unsafe { MaybeUninit::<T>::zeroed().assume_init() },
            outcome: FFIOutcome::Unable,
        }
    }
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

#[ffi_type]
#[repr(C)]
pub struct BattleViewModelRef<'a> {
    pub view_model: &'a BattleViewModel,
}