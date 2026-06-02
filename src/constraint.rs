use crate::error::check_ampl_error;
use crate::ffi;
use crate::ampl::Ampl;
use crate::constraintinstance::Constraintinstance;
use crate::dataframe::DataFrame;
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// An AMPL constraint entity.
///
/// Obtained via [`Ampl::get_constraint`] or [`Ampl::get_constraints`].
pub struct Constraint {
    pub(crate) raw: *mut ffi::AMPL,
    pub(crate) name: String,
}

impl Constraint {
    pub fn new(ampl: &mut Ampl, name: String) -> Self {
        Constraint { raw: ampl.raw, name }
    }

    /// Print the constraint name to stdout.
    pub fn print(&self) {
        println!("Constraint: {}", self.name);
    }

    /// Return true if this is a logical constraint (as opposed to an algebraic one).
    pub fn is_logical(&self) -> bool {
        let name = CString::new(&*self.name).unwrap();
        let mut is_logical: bool = false;
        let err = unsafe { ffi::AMPL_ConstraintIsLogical(self.raw, name.as_ptr(), &mut is_logical) };
        unsafe { check_ampl_error(err) };
        is_logical
    }

    /// Return the instance of this scalar (non-indexed) constraint.
    pub fn get_scalar(&self) -> Constraintinstance {
        let tuple = unsafe {
            let mut t: *mut ffi::AMPL_TUPLE = ptr::null_mut();
            ffi::AMPL_TupleCreate(&mut t, 0, ptr::null_mut());
            Tuple { raw: t }
        };
        Constraintinstance { raw: self.raw, name: self.name.clone(), tuple }
    }

    /// Return the instance indexed by the given string key.
    pub fn get(&self, key: &str) -> Constraintinstance {
        let cs = CString::new(key).unwrap();
        let p = cs.as_ptr();
        let tuple = unsafe {
            let mut t: *mut ffi::AMPL_TUPLE = ptr::null_mut();
            ffi::AMPL_TupleCreateString(&mut t, 1, &p);
            Tuple { raw: t }
        };
        Constraintinstance { raw: self.raw, name: self.name.clone(), tuple }
    }

    /// Return the instance indexed by the given integer key.
    pub fn get_int(&self, key: i64) -> Constraintinstance {
        let tuple = unsafe {
            let mut t: *mut ffi::AMPL_TUPLE = ptr::null_mut();
            let v = key as f64;
            ffi::AMPL_TupleCreateNumeric(&mut t, 1, &mut { v } as *mut f64);
            Tuple { raw: t }
        };
        Constraintinstance { raw: self.raw, name: self.name.clone(), tuple }
    }

    /// Return all instances of this constraint as a list.
    pub fn instances(&self) -> Vec<Constraintinstance> {
        let name = CString::new(&*self.name).unwrap();
        let mut instances_ptr: *mut *mut ffi::AMPL_TUPLE = ptr::null_mut();
        let mut num_instances: usize = 0;
        unsafe {
            let err = ffi::AMPL_EntityGetTuples(self.raw, name.as_ptr(), &mut instances_ptr, &mut num_instances);
            check_ampl_error(err);
            (0..num_instances).map(|i| {
                let tuple = Tuple { raw: *instances_ptr.add(i) };
                Constraintinstance { raw: self.raw, name: self.name.clone(), tuple }
            }).collect()
        }
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
        let err = unsafe { ffi::AMPL_EntityGetIndexarity(self.raw, name.as_ptr(), &mut indexarity) };
        unsafe { check_ampl_error(err) };
        indexarity
    }

    /// Return the total number of instances (rows) of this constraint.
    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        let err = unsafe { ffi::AMPL_EntityGetNumInstances(self.raw, name.as_ptr(), &mut num_instances) };
        unsafe { check_ampl_error(err) };
        num_instances
    }

    /// Return all values and suffixes of this constraint as a DataFrame.
    pub fn get_values(&self) -> DataFrame {
        let name = CString::new(&*self.name).unwrap();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        let err = unsafe {
            ffi::AMPL_EntityGetValues(self.raw, name.as_ptr(), ptr::null(), 0, &mut df)
        };
        unsafe { check_ampl_error(err) };
        DataFrame { raw: df }
    }

    /// Return selected suffixes of this constraint as a DataFrame.
    pub fn get_values_with(&self, suffixes: &[&str]) -> DataFrame {
        let name = CString::new(&*self.name).unwrap();
        let cstrings: Vec<CString> = suffixes.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        let err = unsafe {
            ffi::AMPL_EntityGetValues(self.raw, name.as_ptr(), ptrs.as_ptr(), suffixes.len(), &mut df)
        };
        unsafe { check_ampl_error(err) };
        DataFrame { raw: df }
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
