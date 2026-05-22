use crate::ffi;
use crate::ampl::Ampl;
use crate::variableinstance::Variableinstance;
use crate::tuple::Tuple;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

pub struct Variable {
    pub(crate) ampl: *mut Ampl,
    pub(crate) name: String,
}

impl Variable {
    pub fn print(&self) {
        println!("Variable: {}", self.name);
    }

    pub fn value(&self) -> f64 {
        let name = CString::new(&*self.name).unwrap();
        let mut value: f64 = 0.0;
        unsafe { ffi::AMPL_VariableGetValue((*self.ampl).raw, name.as_ptr(), &mut value as *mut f64) };
        value
    }

    pub fn fix(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableFix((*self.ampl).raw, name.as_ptr()) };
    }

    pub fn fix_with_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableFixWithValue((*self.ampl).raw, name.as_ptr(), value) };
    }

    pub fn unfix(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableUnfix((*self.ampl).raw, name.as_ptr()) };
    }

    pub fn set_value(&self, value: f64) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_VariableSetValue((*self.ampl).raw, name.as_ptr(), value) };
    }

    pub fn indexarity(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut indexarity: usize = 0;
        unsafe { ffi::AMPL_EntityGetIndexarity((*self.ampl).raw, name.as_ptr(), &mut indexarity as *mut usize) };
        indexarity
    }

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

    pub fn num_instances(&self) -> usize {
        let name = CString::new(&*self.name).unwrap();
        let mut num_instances: usize = 0;
        unsafe { ffi::AMPL_EntityGetNumInstances((*self.ampl).raw, name.as_ptr(), &mut num_instances as *mut usize) };
        num_instances
    }

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

    pub fn drop(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityDrop((*self.ampl).raw, name.as_ptr()) };
    }

    pub fn restore(&self) {
        let name = CString::new(&*self.name).unwrap();
        unsafe { ffi::AMPL_EntityRestore((*self.ampl).raw, name.as_ptr()) };
    }
}
