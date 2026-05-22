use crate::ffi;
use crate::suffix::Numericsuffix;
use crate::suffix::Stringsuffix;

use libc::c_char;
use std::ffi::{CStr, CString, c_int};
use std::ptr;

pub struct Objective {
  pub(crate) raw: *mut ffi::AMPL,
  pub(crate) name: String,
}

impl Objective {
  pub fn print(&self) {
    println!("Objective: {}", self.name);
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

  pub fn value(&self) -> f64 {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Numericsuffix::from(Numericsuffix::Value);
    let mut value: f64 = 0.0;
    unsafe { ffi::AMPL_InstanceGetDoubleSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value as *mut f64) };
    value
  }

  pub fn astatus(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Astatus);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  pub fn sstatus(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Sstatus);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  pub fn exitcode(&self) -> i32 {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Numericsuffix::from(Numericsuffix::Exitcode);
    let mut value: c_int = 0;
    unsafe {
      ffi::AMPL_InstanceGetIntSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value);
    }
    value
  }

  pub fn message(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Message);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  pub fn result(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Result);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  pub fn is_minimization(&self) -> bool {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Sense);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      //if value_ptr.is_null() {
      //  return String::new();
      //}
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str == "minimize"
    }
  }
}
