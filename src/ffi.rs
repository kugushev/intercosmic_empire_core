use std::ptr::null_mut;
use crate::ffi::utils::*;
use interoptopus::{ffi_function};
use crate::app::{AppContext, AppSettings};

pub mod utils;
pub mod surrogates;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_init_app(context: &mut *mut AppContext, settings: AppSettings) -> FFIOutcome {
    if !context.is_null() {
        return FFIOutcome::Unable;
    }

    let boxed = Box::new(AppContext::new(settings));

    *context = Box::into_raw(boxed);

    FFIOutcome::Ok
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_close_app(context: &mut *mut AppContext) -> FFIOutcome {
    if context.is_null() {
        return FFIOutcome::Unable;
    }

    {
        let context = unsafe { Box::from_raw(*context) };
        drop(context);
    }

    *context = null_mut();

    FFIOutcome::Ok
}