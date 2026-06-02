use crate::ffi;
use std::ffi::CStr;
use std::panic;

pub(crate) unsafe fn check_ampl_error(err: *mut ffi::AMPL_ERRORINFO) {
    if err.is_null() {
        return;
    }
    let msg_ptr = unsafe { ffi::AMPL_ErrorInfoGetMessage(err) };
    let msg = if msg_ptr.is_null() {
        "unknown AMPL error".to_string()
    } else {
        unsafe { CStr::from_ptr(msg_ptr) }.to_str().unwrap_or("unknown AMPL error").to_owned()
    };
    panic!("{}", msg);
}

/// Run `f`, catching any AMPL panic.
///
/// The default panic hook is suppressed during the call so the error is not
/// printed to stderr twice when the caller handles it.  Returns `Some(msg)` on
/// error, `None` on success.
pub fn catch_ampl_error<F: FnOnce()>(f: F) -> Option<String> {
    let prev = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(panic::AssertUnwindSafe(f));
    panic::set_hook(prev);
    match result {
        Ok(_) => None,
        Err(e) => {
            let msg = e.downcast::<String>()
                .map(|s| *s)
                .or_else(|e| e.downcast::<&'static str>().map(|s| s.to_string()))
                .unwrap_or_else(|_| "unknown AMPL error".to_string());
            Some(msg)
        }
    }
}
