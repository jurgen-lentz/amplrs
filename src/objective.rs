use crate::error::check_ampl_error;
use crate::ffi;
use crate::suffix::Numericsuffix;
use crate::suffix::Stringsuffix;

use libc::c_char;
use std::ffi::{CStr, CString, c_int};
use std::ptr;

/// An AMPL objective entity.
///
/// Obtained via [`Ampl::get_objective`] or [`Ampl::get_objectives`].
pub struct Objective {
  pub(crate) raw: *mut ffi::AMPL,
  pub(crate) name: String,
}

impl Objective {
  /// Print the objective name to stdout.
  pub fn print(&self) {
    println!("Objective: {}", self.name);
  }

  /// Return the indexarity (number of indices) of this objective.
  pub fn indexarity(&self) -> usize {
    let name = CString::new(&*self.name).unwrap();
    let mut indexarity: usize = 0;
    let err = unsafe { ffi::AMPL_EntityGetIndexarity(self.raw, name.as_ptr(), &mut indexarity as *mut usize) };
    unsafe { check_ampl_error(err) };
    indexarity
  }

  /// Return the total number of instances of this objective.
  pub fn num_instances(&self) -> usize {
    let name = CString::new(&*self.name).unwrap();
    let mut num_instances: usize = 0;
    let err = unsafe { ffi::AMPL_EntityGetNumInstances(self.raw, name.as_ptr(), &mut num_instances as *mut usize) };
    unsafe { check_ampl_error(err) };
    num_instances
  }

  /// Return the AMPL declaration string for this objective.
  pub fn declaration(&mut self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_EntityGetDeclaration(self.raw, name.as_ptr(), &mut value_ptr);
      check_ampl_error(err);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  /// Drop this objective from the current model.
  pub fn drop(&self) {
    let name = CString::new(&*self.name).unwrap();
    let err = unsafe { ffi::AMPL_EntityDrop(self.raw, name.as_ptr()) };
    unsafe { check_ampl_error(err) };
  }

  /// Restore a previously dropped objective.
  pub fn restore(&self) {
    let name = CString::new(&*self.name).unwrap();
    let err = unsafe { ffi::AMPL_EntityRestore(self.raw, name.as_ptr()) };
    unsafe { check_ampl_error(err) };
  }

  /// Return the current objective value (`.val` suffix) for a scalar objective.
  pub fn value(&self) -> f64 {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Numericsuffix::from(Numericsuffix::Value);
    let mut value: f64 = 0.0;
    let err = unsafe { ffi::AMPL_InstanceGetDoubleSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value as *mut f64) };
    unsafe { check_ampl_error(err) };
    value
  }

  /// Return the algebraic status (`.astatus`) of this objective.
  pub fn astatus(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Astatus);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      check_ampl_error(err);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  /// Return the solver status (`.sstatus`) of this objective.
  pub fn sstatus(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Sstatus);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      check_ampl_error(err);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  /// Return the solver exit code (`.exitcode`) for this objective.
  pub fn exitcode(&self) -> i32 {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Numericsuffix::from(Numericsuffix::Exitcode);
    let mut value: c_int = 0;
    let err = unsafe {
      ffi::AMPL_InstanceGetIntSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value)
    };
    unsafe { check_ampl_error(err) };
    value
  }

  /// Return the solver message (`.message`) for this objective.
  pub fn message(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Message);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      check_ampl_error(err);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  /// Return the solve result string (`.result`) for this objective.
  pub fn result(&self) -> String {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Result);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      check_ampl_error(err);
      if value_ptr.is_null() {
        return String::new();
      }
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str
    }
  }

  /// Return `true` if this objective is declared as a minimization.
  pub fn is_minimization(&self) -> bool {
    let name = CString::new(&*self.name).unwrap();
    let suffix_c = Stringsuffix::from(Stringsuffix::Sense);
    let mut value_ptr: *mut c_char = ptr::null_mut();
    unsafe {
      let err = ffi::AMPL_InstanceGetStringSuffix(self.raw, name.as_ptr(), std::ptr::null_mut(), suffix_c.into(), &mut value_ptr);
      check_ampl_error(err);
      let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
      ffi::AMPL_StringFree(&mut value_ptr);
      value_str == "minimize"
    }
  }
}
