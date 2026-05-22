use crate::ffi;
use crate::constraint::Constraint;
use crate::dataframe::DataFrame;
use crate::objective::Objective;
use crate::parameter::Parameter;
use crate::set::Set;
use crate::variable::Variable;
extern crate libc;

use libc::c_char;
//use std::ffi::{c_int, CStr, CString};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem::MaybeUninit;

/// Represents of AMPL.
pub struct Ampl {
    pub(crate) raw: *mut ffi::AMPL,
}

impl Ampl {
    pub fn new() -> Self {
        let mut ampl = MaybeUninit::uninit();
        unsafe { ffi::AMPL_Create(ampl.as_mut_ptr()) };
        let ampl = unsafe { ampl.assume_init() };
        Ampl { raw: ampl }
    }

    pub fn clone(&self) -> Self {
        Ampl {
            raw: self.raw,
        }
    }

    pub fn eval(&mut self, statement: &str) {
        let statement = CString::new(statement).unwrap();
        unsafe { ffi::AMPL_Eval(self.raw, statement.as_ptr()) };
    }

    pub fn solve(&self, problem: &str, solver: &str) {
        let problem = CString::new(problem).unwrap();
        let solver = CString::new(solver).unwrap();
        unsafe { ffi::AMPL_Solve(self.raw, problem.as_ptr(), solver.as_ptr()) };
    }

    pub fn reset(&mut self) {
        unsafe { ffi::AMPL_Reset(self.raw) };
    }

    pub fn close(&mut self) {
        unsafe { ffi::AMPL_Close(self.raw) };
    }

    pub fn is_running(&mut self) -> bool {
        let mut running: bool = false;
        unsafe { ffi::AMPL_IsRunning(self.raw, &mut running as *mut bool); }
        running
    }

    pub fn is_busy(&mut self) -> bool {
        let mut busy: bool = false;
        unsafe { ffi::AMPL_IsBusy(self.raw, &mut busy as *mut bool); }
        busy
    }

    pub fn interrupt(&mut self) {
        unsafe { ffi::AMPL_Interrupt(self.raw) };
    }

    pub fn snapshot(&mut self, filename: &str, model: bool, data: bool, options: bool) -> String {
        let filename = CString::new(filename).unwrap();
        let mut snapshot_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_Snapshot(self.raw, filename.as_ptr(), model, data, options, &mut snapshot_ptr);
            if snapshot_ptr.is_null() {
                return String::new();
            }
            let snapshot_str = CStr::from_ptr(snapshot_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut snapshot_ptr);
            snapshot_str
        }
    }

    pub fn export_model(&mut self, filename: &str) -> String {
        let filename = CString::new(filename).unwrap();
        let mut model_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_ExportModel(self.raw, filename.as_ptr(), &mut model_ptr);
            if model_ptr.is_null() {
                return String::new();
            }
            let model_str = CStr::from_ptr(model_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut model_ptr);
            model_str
        }
    }

    pub fn export_data(&mut self, filename: &str) -> String {
        let filename = CString::new(filename).unwrap();
        let mut data_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_ExportData(self.raw, filename.as_ptr(), &mut data_ptr);
            if data_ptr.is_null() {
                return String::new();
            }
            let data_str = CStr::from_ptr(data_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut data_ptr);
            data_str
        }
    }

    pub fn get_current_objective(&mut self) -> String {
        let mut objective_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_GetCurrentObjective(self.raw, &mut objective_ptr);
            if objective_ptr.is_null() {
                return String::new();
            }
            let objective_str = CStr::from_ptr(objective_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut objective_ptr);
            objective_str
        }
    }

    pub fn set_option(&mut self, option: &str, value: &str) {
        let option = CString::new(option).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { ffi::AMPL_SetOption(self.raw, option.as_ptr(), value.as_ptr()) };
    }

    pub fn set_bool_option(&mut self, option: &str, value: bool) {
        let option = CString::new(option).unwrap();
        unsafe { ffi::AMPL_SetBoolOption(self.raw, option.as_ptr(), value) };
    }

    pub fn set_int_option(&mut self, option: &str, value: i32) {
        let option = CString::new(option).unwrap();
        unsafe { ffi::AMPL_SetIntOption(self.raw, option.as_ptr(), value) };
    }

    pub fn set_dbl_option(&mut self, option: &str, value: f64) {
        let option = CString::new(option).unwrap();
        unsafe { ffi::AMPL_SetDblOption(self.raw, option.as_ptr(), value) };
    }

    pub fn get_option(&mut self, option: &str) -> String {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_GetOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value_ptr);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    pub fn get_bool_option(&mut self, option: &str) -> bool {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: bool = false;
        unsafe { ffi::AMPL_GetBoolOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut bool) };
        value
    }

    pub fn get_int_option(&mut self, option: &str) -> i32 {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: i32 = 0;
        unsafe { ffi::AMPL_GetIntOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut i32) };
        value
    }

    pub fn get_dbl_option(&mut self, option: &str) -> f64 {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: f64 = 0.0;
        unsafe { ffi::AMPL_GetDblOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut f64) };
        value
    }

    pub fn read(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::AMPL_Read(self.raw, filename.as_ptr()) };
    }

    pub fn read_data(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe { ffi::AMPL_ReadData(self.raw, filename.as_ptr()) };
    }

    pub fn read_table(&mut self, tablename: &str) {
        let tablename = CString::new(tablename).unwrap();
        unsafe { ffi::AMPL_ReadTable(self.raw, tablename.as_ptr()) };
    }

    pub fn write_table(&mut self, tablename: &str) {
        let tablename = CString::new(tablename).unwrap();
        unsafe { ffi::AMPL_WriteTable(self.raw, tablename.as_ptr()) };
    }

    pub fn write(&mut self, filename: &str, auxfiles: &str) {
        let filename = CString::new(filename).unwrap();
        let auxfiles = CString::new(auxfiles).unwrap();
        unsafe { ffi::AMPL_Write(self.raw, filename.as_ptr(), auxfiles.as_ptr()) };
    }

    pub fn get_constraints(&mut self) -> Vec<Constraint> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        
        unsafe { ffi::AMPL_GetConstraints(self.raw, &mut size, &mut names) };
        
        let mut constraints = Vec::with_capacity(size);
        
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                constraints.push(Constraint::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            
            // Free the allocated memory for names
            libc::free(names as *mut libc::c_void);
        }
        constraints
    }

    pub fn get_objective(&mut self, name: &str) -> Objective {
        Objective {raw: self.raw, name: name.to_string()}
    }

    pub fn get_objectives(&mut self) -> Vec<Objective> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        
        unsafe { ffi::AMPL_GetObjectives(self.raw, &mut size, &mut names) };
        
        let mut objectives = Vec::with_capacity(size);
        
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                objectives.push(Objective {raw: self.raw, name: name_string});
                ffi::AMPL_StringFree(names.add(i));
            }
            
            // Free the allocated memory for names
            libc::free(names as *mut libc::c_void);
        }
        objectives
    }

    pub fn get_parameter(&mut self, name: &str) -> Parameter {
        Parameter::new(self, name.to_string())
    }

    pub fn get_parameters(&mut self) -> Vec<Parameter> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        
        unsafe { ffi::AMPL_GetParameters(self.raw, &mut size, &mut names) };
        
        let mut parameters = Vec::with_capacity(size);
        
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                parameters.push(Parameter::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            
            // Free the allocated memory for names
            libc::free(names as *mut libc::c_void);
        }
        parameters
    }

    pub fn get_sets(&mut self) -> Vec<Set> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        
        unsafe { ffi::AMPL_GetSets(self.raw, &mut size, &mut names) };
        
        let mut sets = Vec::with_capacity(size);
        
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                sets.push(Set::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            
            // Free the allocated memory for names
            libc::free(names as *mut libc::c_void);
        }
        sets
    }

    /// Assign data from a DataFrame to AMPL entities.
    /// If `set_name` is `Some("S")`, the index column(s) are also assigned to set `S`.
    pub fn set_data(&mut self, df: &DataFrame, set_name: Option<&str>) {
        let set_name_c = set_name.map(|s| CString::new(s).unwrap());
        let set_name_ptr = set_name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        unsafe { ffi::AMPL_SetData(self.raw, df.raw, set_name_ptr) };
    }

    /// Retrieve data from AMPL for the given display statements, returned as a DataFrame.
    pub fn get_data(&mut self, statements: &[&str]) -> DataFrame {
        let cstrings: Vec<CString> = statements.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let ptrs: Vec<*const libc::c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        unsafe { ffi::AMPL_GetData(self.raw, ptrs.as_ptr(), statements.len(), &mut df) };
        DataFrame { raw: df }
    }

    pub fn get_variable(&mut self, name: &str) -> Variable {
        Variable {ampl: self, name: name.to_string()}
    }

    pub fn get_variables(&mut self) -> Vec<Variable> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        
        unsafe { ffi::AMPL_GetVariables(self.raw, &mut size, &mut names) };
        
        let mut variables = Vec::with_capacity(size);
        
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                variables.push(Variable {ampl: self, name: name_string});
                ffi::AMPL_StringFree(names.add(i));
            }
            
            // Free the allocated memory for names
            libc::free(names as *mut libc::c_void);
        }
        variables
    }
}

impl Drop for Ampl {
    fn drop(&mut self) {
        // free AMPL instance
        unsafe { ffi::AMPL_Free(&mut self.raw) };
    }
}
