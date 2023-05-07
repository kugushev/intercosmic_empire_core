use std::mem::MaybeUninit;
use interoptopus::{callback, ffi_type};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use crate::game::core::models::stellar_system::{Planet, StellarSystemId, StellarSystemParameters, Sun};
use crate::game::battle::models::battle_state::BattleState;
use crate::game::battle::models::warp_gate::WarpGate;

callback!(FFILog(log: AsciiPointer) -> u8);

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
    pub planets: FFISlice<'a, Planet>
}


#[ffi_type]
#[repr(C)]
pub struct BattleStateViewModel<'a> {
    pub warp_gates: FFISlice<'a, WarpGate>
}

impl<'a> BattleStateViewModel<'a> {
    pub fn from(view_state: &BattleState) -> BattleStateViewModel { BattleStateViewModel {
        warp_gates: FFISlice::from_slice(&view_state.warp_gates)
    }}
}

impl FFIOutcome {
    pub fn assert(&self, expected: FFIOutcome){
        assert_eq!(*self, expected)
    }
}

#[ffi_type]
#[repr(C)]
#[derive(Eq, PartialEq, Hash)]
pub enum RouteBuildersSource{
    LeftHand,
    RightHand,
    Ai
}
