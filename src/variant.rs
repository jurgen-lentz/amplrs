use crate::ffi;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::ptr;

pub struct Variant {
    raw: *mut ffi::AMPL_VARIANT,
}

impl Variant {
    pub fn new() -> Self {
        let mut variant = MaybeUninit::uninit();
        unsafe { ffi::AMPL_VariantCreateEmpty(variant.as_mut_ptr()) };
        let variant = unsafe { variant.assume_init() };
        Variant { raw: variant }
    }

    pub fn new_from_string(value: &str) -> Self {
        let value = CString::new(value).unwrap();
        let mut variant = MaybeUninit::uninit();
        unsafe { ffi::AMPL_VariantCreateString(variant.as_mut_ptr(), value.as_ptr()) };
        let variant = unsafe { variant.assume_init() };
        Variant { raw: variant }
    }

    pub fn new_from_double(value: f64) -> Self {
        let mut variant = MaybeUninit::uninit();
        unsafe { ffi::AMPL_VariantCreateNumeric(variant.as_mut_ptr(), value) };
        let variant = unsafe { variant.assume_init() };
        Variant { raw: variant }
    }

    pub fn get_numeric(&self) -> f64 {
        let mut value: f64 = 0.0;
        unsafe { ffi::AMPL_VariantGetNumericValue(self.raw, &mut value as *mut f64) };
        value
    }

    pub fn get_string(&self) -> String {
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_VariantGetStringValue(self.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    pub fn format(&self) -> String {
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_VariantFormat(self.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

}
