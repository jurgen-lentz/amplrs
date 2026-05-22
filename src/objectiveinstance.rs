use crate::ffi;
use crate::ampl::Ampl;
use crate::suffix::{Numericsuffix, Stringsuffix};
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Objectiveinstance {
    pub(crate) ampl: *mut Ampl,
    pub(crate) name: String,
    pub(crate) tuple: Tuple,
}

impl Objectiveinstance {
    pub(crate) fn new(ampl: &mut Ampl, name: String, tuple: Tuple) -> Self {
        Objectiveinstance { ampl: ampl, name: name, tuple: tuple }
    }

    pub fn name(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_InstanceGetName((*self.ampl).raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    pub fn to_string(&self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_InstanceToString((*self.ampl).raw, name.as_ptr(), self.tuple.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe {
            ffi::AMPL_InstanceDrop((*self.ampl).raw, name.as_ptr(), self.tuple.raw);
        }
    }

    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe {
            ffi::AMPL_InstanceRestore((*self.ampl).raw, name.as_ptr(), self.tuple.raw);
        }
    }

    pub fn dbl_suffix(&self, suffix: Numericsuffix) -> f64 {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Numericsuffix::from(suffix);
        let mut value: f64 = 0.0;
        unsafe {
            ffi::AMPL_InstanceGetDoubleSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value as *mut f64);
        }
        value
    }

    pub fn int_suffix(&self, suffix: Numericsuffix) -> i32 {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Numericsuffix::from(suffix);
        let mut value: i32 = 0;
        unsafe {
            ffi::AMPL_InstanceGetIntSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value as *mut i32);
        }
        value
    }

    pub fn string_suffix(&self, suffix: Stringsuffix) -> String {
        let name = CString::new(&*self.name).unwrap();
        let suffix_c = Stringsuffix::from(suffix);
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_InstanceGetStringSuffix((*self.ampl).raw, name.as_ptr(), self.tuple.raw, suffix_c.into(), &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }
}
