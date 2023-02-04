use std::ffi::CString;
use interoptopus::patterns::string::AsciiPointer;
use bevy_ecs::prelude::Resource;
use crate::ffi_models::FFILog;

#[derive(Resource)]
pub struct InteropLogger {
    pub ffi_log: FFILog,
    pub trace_enabled: bool,
    msg_owner: CString,
}

impl Default for InteropLogger {
    fn default() -> Self {
        Self {
            ffi_log: FFILog::default(),
            trace_enabled: false,
            msg_owner: CString::default(),
        }
    }
}

impl InteropLogger {
    // todo: create macro to reduce allocation
    pub fn log(&mut self, entry: String) {
        self.msg_owner = CString::new(entry).expect("CString::new failed");
        self.ffi_log.call_if_some(AsciiPointer::from_cstr(&self.msg_owner));
    }
}