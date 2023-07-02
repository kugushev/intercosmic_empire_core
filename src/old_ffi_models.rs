use std::mem::MaybeUninit;
use glam::Quat;
use interoptopus::{callback, ffi_type};
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;
use crate::old::core::models::stellar_system::{Planet, StellarSystemId, StellarSystemParameters, Sun};
use crate::old::battle::models::battle_state::BattleState;
use crate::old::battle::models::warp_gate::WarpGate;
use crate::old::battle::views::{BattleViewsState, SpaceshipViewModel};

callback!(FFILog(log: AsciiPointer) -> u8);

pub fn ffi_log_println() -> FFILog{
    FFILog(Some(log))
}

extern "C" fn log(log: AsciiPointer) -> u8 {
    println!("{}", log.as_str().unwrap());
    0
}

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
}


#[ffi_type]
#[repr(C)]
pub struct BattleStateViewModel<'a> {
    pub warp_gates: FFISlice<'a, WarpGate>,
    pub spaceships: FFISlice<'a, SpaceshipViewModel>
}

impl<'a> BattleStateViewModel<'a> {
    pub fn from(battle_state: &'a BattleState, battle_view_state: &'a BattleViewsState) -> BattleStateViewModel<'a>
    {
        BattleStateViewModel {
            warp_gates: FFISlice::from_slice(&battle_state.warp_gates),
            spaceships: FFISlice::from_slice(&battle_view_state.spaceships),
        }
    }
}

impl FFIOutcome {
    pub fn assert(&self, expected: FFIOutcome) {
        assert_eq!(*self, expected)
    }
}

#[ffi_type]
#[repr(C)]
#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum RouteBuildersSource {
    LeftHand,
    RightHand,
    Ai,
}

#[ffi_type(name = "Quaternion", namespace = "UnityEngine")]
#[repr(C)]
#[derive(Clone)]
pub struct FFIQuat{
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl From<Quat> for FFIQuat {
    fn from(value: Quat) -> Self {
        // glam::Quat is not repr(C) so we can't use bytes as is
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w
        }
    }
}