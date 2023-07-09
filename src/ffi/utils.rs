use std::mem::MaybeUninit;
use glam::Quat;
use interoptopus::{callback, ffi_type};
use interoptopus::patterns::string::AsciiPointer;
use crate::app::utils::zero;

callback!(FFILog(log: AsciiPointer) -> u8);

pub fn ffi_log_println() -> FFILog {
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
    // Can't do, no errors in logs
    Unable,
    // Error occurs, go to guard for logs
    Error,
    // Panic occurs, go to guard for logs
    Panic,
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

    pub fn error() -> FFIResult<T> {
        FFIResult {
            value: unsafe { MaybeUninit::<T>::zeroed().assume_init() },
            outcome: FFIOutcome::Error,
        }
    }

    pub fn panic() -> FFIResult<T> {
        FFIResult {
            value: zero(),
            outcome: FFIOutcome::Panic,
        }
    }
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