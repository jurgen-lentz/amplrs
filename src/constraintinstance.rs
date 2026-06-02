use crate::error::check_ampl_error;
use crate::ffi;
use crate::suffix::{Numericsuffix, Stringsuffix};
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// A single instance of an AMPL constraint, identified by its indexing tuple.
pub struct Constraintinstance {
    pub(crate) raw: *mut ffi::AMPL,
    pub(crate) name: String,
    pub(crate) tuple: Tuple,
}

impl Constraintinstance {
    fn dbl(&self, suffix: Numericsuffix) -> f64 {
        let name = CString::new(&*self.name).unwrap();
        let s: ffi::AMPL_NUMERICSUFFIX = suffix.into();
        let mut value: f64 = 0.0;
        let err = unsafe {
            ffi::AMPL_InstanceGetDoubleSuffix(self.raw, name.as_ptr(), self.tuple.raw, s, &mut value)
        };
        unsafe { check_ampl_error(err) };
        value
    }

    fn str_suffix(&self, suffix: Stringsuffix) -> String {
        let name = CString::new(&*self.name).unwrap();
        let s: ffi::AMPL_STRINGSUFFIX = suffix.into();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), self.tuple.raw, s, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let s = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            s
        }
    }

    /// Return a human-readable string representation of this constraint instance.
    pub fn to_string(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_InstanceToString(self.raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let s = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            s
        }
    }

    /// Return the fully-qualified AMPL name of this instance (e.g. `"c['a']"`).
    pub fn name(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_InstanceGetName(self.raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let s = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            s
        }
    }

    // ── numeric suffixes ─────────────────────────────────────────────────────

    pub fn body(&self)   -> f64 { self.dbl(Numericsuffix::Body) }
    pub fn dual(&self)   -> f64 { self.dbl(Numericsuffix::Dual) }
    pub fn lb(&self)     -> f64 { self.dbl(Numericsuffix::Lb) }
    pub fn ub(&self)     -> f64 { self.dbl(Numericsuffix::Ub) }
    pub fn lbs(&self)    -> f64 { self.dbl(Numericsuffix::Lbs) }
    pub fn ubs(&self)    -> f64 { self.dbl(Numericsuffix::Ubs) }
    pub fn ldual(&self)  -> f64 { self.dbl(Numericsuffix::Ldual) }
    pub fn udual(&self)  -> f64 { self.dbl(Numericsuffix::Udual) }
    pub fn lslack(&self) -> f64 { self.dbl(Numericsuffix::Lslack) }
    pub fn uslack(&self) -> f64 { self.dbl(Numericsuffix::Uslack) }
    pub fn slack(&self)  -> f64 { self.dbl(Numericsuffix::Slack) }
    pub fn defvar(&self) -> f64 { self.dbl(Numericsuffix::Defvar) }
    pub fn dinit(&self)  -> f64 { self.dbl(Numericsuffix::Dinit) }
    pub fn dinit0(&self) -> f64 { self.dbl(Numericsuffix::Dinit0) }
    /// Value of a logical constraint (`.val`).
    pub fn val(&self)    -> f64 { self.dbl(Numericsuffix::Val) }

    // ── string suffixes ──────────────────────────────────────────────────────

    pub fn astatus(&self) -> String { self.str_suffix(Stringsuffix::Astatus) }
    pub fn sstatus(&self) -> String { self.str_suffix(Stringsuffix::Sstatus) }
    pub fn status(&self)  -> String { self.str_suffix(Stringsuffix::Status) }

    // ── mutation / control ───────────────────────────────────────────────────

    /// Set the dual variable value for this constraint instance.
    pub fn set_dual(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe {
            ffi::AMPL_ConstraintInstanceSetDual(self.raw, name.as_ptr(), self.tuple.raw, value)
        };
        unsafe { check_ampl_error(err) };
    }

    /// Drop this constraint instance from the active model.
    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_InstanceDrop(self.raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Restore a previously dropped constraint instance.
    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_InstanceRestore(self.raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Return the value of a numeric suffix for this constraint instance.
    pub fn dbl_suffix(&self, suffix: Numericsuffix) -> f64 { self.dbl(suffix) }

    /// Return the value of an integer numeric suffix for this constraint instance.
    pub fn int_suffix(&self, suffix: Numericsuffix) -> i32 {
        let name = CString::new(&*self.name).unwrap();
        let s: ffi::AMPL_NUMERICSUFFIX = suffix.into();
        let mut value: i32 = 0;
        let err = unsafe {
            ffi::AMPL_InstanceGetIntSuffix(self.raw, name.as_ptr(), self.tuple.raw, s, &mut value)
        };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Return the value of a string suffix for this constraint instance.
    pub fn string_suffix(&self, suffix: Stringsuffix) -> String { self.str_suffix(suffix) }
}
