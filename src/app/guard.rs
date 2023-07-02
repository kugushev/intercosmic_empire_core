use std::ffi::CString;
use std::panic;
use interoptopus::{ffi_function};
use interoptopus::patterns::string::AsciiPointer;
use crate::app::AppContext;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_last_exception(context: &AppContext) -> AsciiPointer {
    AsciiPointer::from_cstr(&context.guard.last_exception)
}

#[derive(Default)]
pub struct Guard {
    pub last_exception: CString,
}

impl Guard {
    pub fn wrap<T>(&mut self, func: impl FnOnce() -> Result<T, String>) -> FFIResult<T> {
        let unwind_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            func()
        }));

        match unwind_result {
            Ok(result) => {
                match result {
                    Ok(value) => { FFIResult::ok(value) }
                    Err(error) => {
                        self.set_last_exception(error);
                        FFIResult::error()
                    }
                }
            }
            Err(panic) => {
                match panic.downcast::<&str>() {
                    Ok(panic_msg) => {
                        self.set_last_exception((*panic_msg).to_string())
                    }
                    Err(_) => {
                        self.set_last_exception("Panic doesn't have appropriate message".to_string())
                    }
                }
                FFIResult::panic()
            }
        }
    }

    fn set_last_exception(&mut self, msg: String) {
        self.last_exception = CString::new(msg).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use crate::app::guard::Guard;
    use crate::ffi::utils::{FFIOutcome, FFIResult};

    #[test]
    fn guard_ok() {
        let mut guard = Guard::default();
        let result = guard.wrap(|| { Ok(42) });
        assert!(result.outcome == FFIOutcome::Ok && result.value == 42);
    }

    #[test]
    fn guard_err() {
        let mut guard = Guard::default();
        let result: FFIResult<i32> = guard.wrap(|| { Err(String::from("Pain")) });
        assert!(result.outcome == FFIOutcome::Error && guard.last_exception == CString::new("Pain").unwrap());
    }

    #[test]
    fn guard_panic() {
        let mut guard = Guard::default();
        let result: FFIResult<i32> = guard.wrap(|| { panic!("Sadness") });
        assert!(result.outcome == FFIOutcome::Panic && guard.last_exception == CString::new("Sadness").unwrap());
    }
}