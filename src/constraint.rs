use crate::error::check_ampl_error;
use crate::ffi;
use crate::ampl::Ampl;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// An AMPL constraint entity.
///
/// Obtained via [`Ampl::get_constraint`] or [`Ampl::get_constraints`].
pub struct Constraint {
    raw: *mut ffi::AMPL,
    name: String,
}

impl Constraint {
    pub fn new(ampl: &mut Ampl, name: String) -> Self {
        Constraint { raw: ampl.raw, name : name }
    }

    /// Print the constraint name to stdout.
    pub fn print(&self) {
        println!("Constraint: {}", self.name);
    }

    /// Set the dual value for a scalar constraint instance.
    pub fn set_dual(&self, dual: f64) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_ConstraintSetDual(self.raw, name.as_ptr(), dual) };
        unsafe { check_ampl_error(err) };
    }

    /// Return the indexarity (number of indices) of this constraint.
    pub fn indexarity(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut indexarity: usize = 0;
        let err = unsafe { ffi::AMPL_EntityGetIndexarity(self.raw, name.as_ptr(), &mut indexarity as *mut usize) };
        unsafe { check_ampl_error(err) };
        indexarity
    }

    /// Return the total number of instances (rows) of this constraint.
    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        let err = unsafe { ffi::AMPL_EntityGetNumInstances(self.raw, name.as_ptr(), &mut num_instances as *mut usize) };
        unsafe { check_ampl_error(err) };
        num_instances
    }

    /// Return the AMPL declaration string for this constraint.
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

    /// Drop this constraint from the current model (equivalent to AMPL `drop` command).
    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_EntityDrop(self.raw, name.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Restore a previously dropped constraint (equivalent to AMPL `restore` command).
    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_EntityRestore(self.raw, name.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }
}
