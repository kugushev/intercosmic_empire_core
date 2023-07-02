use std::cell::RefCell;
use std::ffi::CString;
use std::rc::{Rc, Weak};
use interoptopus::patterns::string::AsciiPointer;
use bevy_ecs::prelude::Resource;
use crate::old_ffi_models::FFILog;

#[derive(Resource, Default)]
pub struct InteropLogger {
    pub ffi_log: FFILog,
    pub trace_enabled: bool,
    msg_owner: CString,
}

impl InteropLogger {
    pub fn trace(&mut self, entry: String) {
        self.msg_owner = CString::new(entry).expect("CString::new failed");
        self.ffi_log.call_if_some(AsciiPointer::from_cstr(&self.msg_owner));
    }
}

#[macro_export]
macro_rules! trace {
    ($loggerRef:ident, $e:expr) => {
        {
            let cell = $loggerRef.0.upgrade().expect("Unable to unwrap logger RC");
            let mut logger = cell.borrow_mut();
            if logger.trace_enabled {
                let module = module_path!();
                logger.trace(format!("{}: {}", module, $e));
            }
        }
    };
}

pub struct LoggerRef(pub Weak<RefCell<InteropLogger>>);

impl LoggerRef {
    pub fn new(rc: &Rc<RefCell<InteropLogger>>) -> LoggerRef {
        LoggerRef(Rc::downgrade(rc))
    }
}