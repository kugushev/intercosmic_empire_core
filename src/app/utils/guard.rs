use std::cell::RefCell;
use std::ffi::CString;
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};
use backtrace::Backtrace;
use interoptopus::{ffi_function};
use interoptopus::patterns::string::AsciiPointer;
use crate::app::AppContext;
use crate::ffi::utils::FFIResult;

#[ffi_function]
#[no_mangle]
pub extern "C" fn ice_get_last_exception(context: &AppContext) -> AsciiPointer {
    AsciiPointer::from_cstr(&context.guard.last_exception)
}

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
                let msg = match panic.downcast::<&str>() {
                    Ok(panic_msg) => {
                        (*panic_msg).to_string()
                    }
                    Err(_) => {
                        "Panic doesn't have appropriate message".to_string()
                    }
                };

                let b = BACKTRACE.with(|b| b.borrow_mut().take()).unwrap();

                self.set_last_exception(format!("{msg}\nat panic:\n{:?}", b));
                FFIResult::panic()
            }
        }
    }

    fn set_last_exception(&mut self, msg: String) {
        self.last_exception = CString::new(msg).unwrap();
    }
}

static BACKTRACE_IS_SET: AtomicBool = AtomicBool::new(false);

thread_local! {
    static BACKTRACE: RefCell<Option<Backtrace>> = RefCell::new(None);
}

impl Default for Guard {
    fn default() -> Self {
        let acquired = BACKTRACE_IS_SET.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire);
        if acquired.is_ok() {
            panic::set_hook(Box::new(|_| {
                let trace = Backtrace::new();
                BACKTRACE.with(move |b| b.borrow_mut().replace(trace));
            }));
        }

        Self {
            last_exception: CString::default()
        }
    }
}


#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use crate::app::utils::guard::Guard;
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
        assert_eq!(result.outcome, FFIOutcome::Panic);
    }
}