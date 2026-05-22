use crate::ffi;
use crate::ampl::Ampl;
use crate::dataframe::DataFrame;
use crate::variableinstance::Variableinstance;
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// An AMPL variable entity.
///
/// Obtained via [`Ampl::get_variable`] or [`Ampl::get_variables`].
pub struct Variable {
    pub(crate) ampl: *mut Ampl,
    pub(crate) name: String,
}

impl Variable {
    /// Print the variable name to stdout.
    pub fn print(&self) {
        println!("Variable: {}", self.name);
    }

    /// Return the current value for a scalar variable.
    pub fn value(&self) -> f64 {
        let name = CString::new(&*self.name).unwrap();
        let mut value: f64 = 0.0;
        unsafe { ffi::AMPL_VariableGetValue((*self.ampl).raw, name.as_ptr(), &mut value as *mut f64) };
        value
    }

    /// Fix a scalar variable at its current value, removing it from the optimisation.
    pub fn fix(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableFix((*self.ampl).raw, name.as_ptr()) };
    }

    /// Fix a scalar variable at the specified `value`.
    pub fn fix_with_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableFixWithValue((*self.ampl).raw, name.as_ptr(), value) };
    }

    /// Unfix a previously fixed variable, allowing it to be optimised again.
    pub fn unfix(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableUnfix((*self.ampl).raw, name.as_ptr()) };
    }

    /// Set the value of a scalar variable.
    pub fn set_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableSetValue((*self.ampl).raw, name.as_ptr(), value) };
    }

    /// Return the indexarity (number of indices) of this variable.
    pub fn indexarity(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut indexarity: usize = 0;
        unsafe { ffi::AMPL_EntityGetIndexarity((*self.ampl).raw, name.as_ptr(), &mut indexarity as *mut usize) };
        indexarity
    }

    /// Return all instances of this variable as a list of [`Variableinstance`] objects.
    pub fn instances(&self) -> Vec<Variableinstance> {
        let name = CString::new(&*self.name).unwrap();
        let mut instances_ptr: *mut *mut ffi::AMPL_TUPLE = ptr::null_mut();
        let mut num_instances: usize = 0;
        unsafe {
            ffi::AMPL_EntityGetTuples((*self.ampl).raw, name.as_ptr(), &mut instances_ptr, &mut num_instances as *mut usize);
            let mut instances = Vec::new();
            for i in 0..num_instances {
                let raw = *instances_ptr.add(i);
                let tuple = Tuple {raw: raw};
                let instance = Variableinstance::new(&mut *self.ampl, self.name.clone(), tuple);
                instances.push(instance);
            }
            instances
        }
    }

    /// Return the total number of instances of this variable.
    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        unsafe { ffi::AMPL_EntityGetNumInstances((*self.ampl).raw, name.as_ptr(), &mut num_instances as *mut usize) };
        num_instances
    }

    /// Return the AMPL declaration string for this variable.
    pub fn declaration(&mut self) -> String {
        let name = CString::new(&*self.name).unwrap();
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_EntityGetDeclaration((*self.ampl).raw, name.as_ptr(), &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    /// Retrieve all values and suffixes of this variable as a DataFrame.
    pub fn get_values(&self) -> DataFrame {
        let name = CString::new(&*self.name).unwrap();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        unsafe {
            ffi::AMPL_EntityGetValues((*self.ampl).raw, name.as_ptr(), ptr::null(), 0, &mut df);
        }
        DataFrame { raw: df }
    }

    /// Drop this variable from the current model.
    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityDrop((*self.ampl).raw, name.as_ptr()) };
    }

    /// Restore a previously dropped variable.
    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityRestore((*self.ampl).raw, name.as_ptr()) };
    }
}
