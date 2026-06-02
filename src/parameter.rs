use crate::error::check_ampl_error;
use crate::ffi;
use crate::ampl::Ampl;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// An AMPL parameter entity.
///
/// Obtained via [`Ampl::get_parameter`] or [`Ampl::get_parameters`].
pub struct Parameter {
    raw: *mut ffi::AMPL,
    name: String,
}

impl Parameter {
    pub fn new(ampl: &mut Ampl, name: String) -> Self {
        Parameter { raw: ampl.raw, name:name }
    }

    /// Print the parameter name to stdout.
    pub fn print(&self) {
        println!("Parameter: {}", self.name);
    }

    /// Return the indexarity (number of indices) of this parameter.
    pub fn indexarity(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut indexarity: usize = 0;
        let err = unsafe { ffi::AMPL_EntityGetIndexarity(self.raw, name.as_ptr(), &mut indexarity as *mut usize) };
        unsafe { check_ampl_error(err) };
        indexarity
    }

    /// Return the total number of instances of this parameter.
    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        let err = unsafe { ffi::AMPL_EntityGetNumInstances(self.raw, name.as_ptr(), &mut num_instances as *mut usize) };
        unsafe { check_ampl_error(err) };
        num_instances
    }

    /// Return the AMPL declaration string for this parameter.
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

    /// Assign `values` to all instances of this parameter in the order they appear in the model.
    ///
    /// The length of `values` must equal the number of instances.
    pub fn set_all_double_values(&self, values: &[f64]) {
        let name = CString::new(&*self.name).unwrap();
        let mut args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        unsafe {
            ffi::AMPL_ArgsCreateNumeric(&mut args, values.as_ptr());
            let err = ffi::AMPL_ParameterSetArgsValues(
                self.raw,
                name.as_ptr(),
                values.len(),
                args,
            );
            check_ampl_error(err);
            ffi::AMPL_ArgsDestroy(&mut args);
        }
    }

    /// Assign `values` to the specific instances identified by the string `indices`.
    ///
    /// Both slices must have the same length. Each index is treated as a 1-element string tuple.
    pub fn set_some_double_values(&self, indices: &[&str], values: &[f64]) {
        assert_eq!(indices.len(), values.len());
        let name = CString::new(&*self.name).unwrap();

        let mut tuples: Vec<*mut ffi::AMPL_TUPLE> = indices.iter().map(|&s| {
            let cs = CString::new(s).unwrap();
            let p = cs.as_ptr();
            let mut tuple: *mut ffi::AMPL_TUPLE = ptr::null_mut();
            unsafe { ffi::AMPL_TupleCreateString(&mut tuple, 1, &p) };
            std::mem::forget(cs);
            tuple
        }).collect();

        let mut vals = values.to_vec();
        unsafe {
            let err = ffi::AMPL_ParameterSetSomeDoubleValues(
                self.raw,
                name.as_ptr(),
                tuples.len(),
                tuples.as_mut_ptr(),
                vals.as_mut_ptr(),
            );
            check_ampl_error(err);
            for t in &mut tuples {
                ffi::AMPL_TupleFree(t);
            }
        }
    }

    /// Drop this parameter from the current model.
    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_EntityDrop(self.raw, name.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Restore a previously dropped parameter.
    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        let err = unsafe { ffi::AMPL_EntityRestore(self.raw, name.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }
}
