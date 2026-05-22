use crate::ffi;
use std::ffi::CStr;

/// Check an AMPL error pointer returned by a C API call.
///
/// If `err` is non-null the error message is extracted, the struct is freed,
/// and the function panics with that message.  A null pointer means success.
pub(crate) unsafe fn check_ampl_error(mut err: *mut ffi::AMPL_ERRORINFO) {
    if err.is_null() {
        return;
    }
    let msg_ptr = unsafe { ffi::AMPL_ErrorInfoGetMessage(err) };
    let msg = if msg_ptr.is_null() {
        "unknown AMPL error".to_string()
    } else {
        unsafe { CStr::from_ptr(msg_ptr) }.to_str().unwrap_or("unknown AMPL error").to_owned()
    };
    unsafe { ffi::AMPL_ErrorInfoFree(&mut err) };
    panic!("{}", msg);
}
