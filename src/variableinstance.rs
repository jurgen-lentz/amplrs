use crate::error::check_ampl_error;
use crate::ffi;
use crate::ampl::Ampl;
use crate::dataframe::Value;
use crate::suffix::{Numericsuffix, Stringsuffix};
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// A single instance of an AMPL variable, identified by its indexing tuple.
pub struct Variableinstance {
    pub(crate) ampl: *mut Ampl,
    pub(crate) name: String,
    pub(crate) tuple: Tuple,
}

impl Variableinstance {
    pub(crate) fn new(ampl: &mut Ampl, name: String, tuple: Tuple) -> Self {
        Variableinstance { ampl: ampl, name: name, tuple: tuple }
    }

    /// Return the fully-qualified AMPL name of this instance (e.g. `"Buy['BEEF']"`).
    pub fn name(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_InstanceGetName((*self.ampl).raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    /// Return a human-readable string representation of this variable instance.
    pub fn to_string(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_VariableInstanceToString((*self.ampl).raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    /// Drop this variable instance from the active model.
    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_InstanceDrop((*self.ampl).raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Restore a previously dropped variable instance.
    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_InstanceRestore((*self.ampl).raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Return the value of a numeric suffix for this variable instance.
    pub fn dbl_suffix(&self, suffix: Numericsuffix) -> f64 {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Numericsuffix::from(suffix);
        let mut value: f64 = 0.0;
        let err = unsafe {
            ffi::AMPL_InstanceGetDoubleSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value as *mut f64)
        };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Return the value of an integer numeric suffix for this variable instance.
    pub fn int_suffix(&self, suffix: Numericsuffix) -> i32 {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Numericsuffix::from(suffix);
        let mut value: i32 = 0;
        let err = unsafe {
            ffi::AMPL_InstanceGetIntSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value as *mut i32)
        };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Return the value of a string suffix for this variable instance.
    pub fn string_suffix(&self, suffix: Stringsuffix) -> String {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Stringsuffix::from(suffix);
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_InstanceGetStringSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    /// Return the current value of this variable instance.
    pub fn value(&self) -> f64 { self.dbl_suffix(Numericsuffix::Value) }

    /// Return the dual value of this variable instance.
    pub fn dual(&self) -> f64 { self.dbl_suffix(Numericsuffix::Dual) }

    /// Return the index tuple of this instance as a `Vec<Value>`.
    pub fn key(&self) -> Vec<Value> {
        unsafe {
            let mut size: usize = 0;
            ffi::AMPL_TupleGetSize(self.tuple.raw, &mut size);
            (0..size).map(|i| {
                let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
                ffi::AMPL_TupleGetVariant(self.tuple.raw, i, &mut var);
                let mut type_: ffi::AMPL_TYPE = ffi::AMPL_TYPE_AMPL_NUMERIC;
                ffi::AMPL_VariantGetType(var, &mut type_);
                if type_ == ffi::AMPL_TYPE_AMPL_STRING {
                    let mut s_ptr: *mut c_char = ptr::null_mut();
                    ffi::AMPL_VariantGetStringValue(var, &mut s_ptr);
                    let s = if s_ptr.is_null() {
                        String::new()
                    } else {
                        let owned = CStr::from_ptr(s_ptr).to_str().unwrap().to_string();
                        ffi::AMPL_StringFree(&mut s_ptr);
                        owned
                    };
                    ffi::AMPL_VariantFree(&mut var);
                    Value::Text(s)
                } else {
                    let mut v: f64 = 0.0;
                    ffi::AMPL_VariantGetNumericValue(var, &mut v);
                    ffi::AMPL_VariantFree(&mut var);
                    Value::Numeric(v)
                }
            }).collect()
        }
    }

    /// Fix this variable instance at its current value.
    pub fn fix(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_VariableInstanceFix((*self.ampl).raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Fix this variable instance at the specified `value`.
    pub fn fix_to_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_VariableInstanceFixToValue((*self.ampl).raw, name.as_ptr(), self.tuple.raw, value) };
        unsafe { check_ampl_error(err) };
    }

    /// Unfix this variable instance so it can be optimised again.
    pub fn unfix(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_VariableInstanceUnfix((*self.ampl).raw, name.as_ptr(), self.tuple.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Set the value of this variable instance.
    pub fn set_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_VariableInstanceSetValue((*self.ampl).raw, name.as_ptr(), self.tuple.raw, value) };
        unsafe { check_ampl_error(err) };
    }
}
