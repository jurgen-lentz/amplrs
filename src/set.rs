use crate::ffi;
use crate::ampl::Ampl;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Set {
    raw: *mut ffi::AMPL,
    name: String,
}

impl Set {
    pub fn new(ampl: &mut Ampl, name: String) -> Self {
        Set { raw: ampl.raw, name:name }
    }

    pub fn print(&self) {
        println!("Set: {}", self.name);
    }

    pub fn indexarity(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut indexarity: usize = 0;
        unsafe { ffi::AMPL_EntityGetIndexarity(self.raw, name.as_ptr(), &mut indexarity as *mut usize) };
        indexarity
    }

    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        unsafe { ffi::AMPL_EntityGetNumInstances(self.raw, name.as_ptr(), &mut num_instances as *mut usize) };
        num_instances
    }

    pub fn declaration(&mut self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_EntityGetDeclaration(self.raw, name.as_ptr(), &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityDrop(self.raw, name.as_ptr()) };
    }

    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityRestore(self.raw, name.as_ptr()) };
    }
}
