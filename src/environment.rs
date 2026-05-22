use crate::ffi;
extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem::MaybeUninit;

/// Represents of Environment.
pub struct Environment {
    pub(crate) raw: *mut ffi::AMPL_ENVIRONMENT,
}

impl Environment {
    pub fn new(bin_dir: &str , bin_name: &str) -> Self {
        let bin_dir = CString::new(bin_dir).unwrap();
        let bin_name = CString::new(bin_name).unwrap();
        let mut environment = MaybeUninit::uninit();
        unsafe { ffi::AMPL_EnvironmentCreate(environment.as_mut_ptr(), bin_dir.as_ptr(), bin_name.as_ptr()) };
        let environment = unsafe { environment.assume_init() };
        Environment { raw: environment }
    }

    pub fn clone(&self) -> Self {
        Environment {
            raw: self.raw,
        }
    }

    pub fn add_environment_variable(&self, name: &str, value: &str) {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { ffi::AMPL_EnvironmentAddEnvironmentVariable(self.raw, name.as_ptr(), value.as_ptr()) };
    }

    pub fn get_bin_dir(&self) -> String {
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_EnvironmentGetBinaryDirectory(self.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            //libc::free(value_ptr as *mut libc::c_void);
            value_str
        }
    }

    pub fn set_bin_dir(&self, bin_dir: &str) {
        let bin_dir = CString::new(bin_dir).unwrap();
        unsafe { ffi::AMPL_EnvironmentSetBinaryDirectory(self.raw, bin_dir.as_ptr()) };
    }

    pub fn get_bin_name(&self) -> String {
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_EnvironmentGetBinaryName(self.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            //libc::free(value_ptr as *mut libc::c_void);
            value_str
        }
    }

    pub fn set_bin_name(&self, bin_name: &str) {
        let bin_name = CString::new(bin_name).unwrap();
        unsafe { ffi::AMPL_EnvironmentSetBinaryName(self.raw, bin_name.as_ptr()) };
    }

    pub fn to_string(&self) -> String {
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_EnvironmentToString(self.raw, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = String::from(CStr::from_ptr(value_ptr).to_str().unwrap());
            libc::free(value_ptr as *mut libc::c_void);
            value_str
        }
    }

    pub fn size(&self) -> usize {
        let mut value: usize = 0;
        unsafe { ffi::AMPL_EnvironmentGetSize(self.raw, &mut value as *mut usize) };
        value
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        // free AMPL_ENVIRONMENT instance
        unsafe { ffi::AMPL_EnvironmentFree(&mut self.raw) };
    }
}
