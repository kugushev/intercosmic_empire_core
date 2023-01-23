use std::ffi::CString;
use interoptopus::patterns::string::AsciiPointer;
use bevy_ecs::prelude::Resource;
use crate::ffi_models::FFILog;

#[derive(Resource, Default)]
pub struct InteropLogger {
    pub ffi_log: FFILog,
    msg_owner: CString,
}

impl InteropLogger {

    // todo: create macro to reduce allocation
    pub fn log(&mut self, entry: String) {
        self.msg_owner = CString::new(entry).expect("CString::new failed");
        self.ffi_log.call_if_some(AsciiPointer::from_cstr(&self.msg_owner));
    }
}