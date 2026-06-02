use crate::error::check_ampl_error;
use crate::ffi;
use crate::constraint::Constraint;
use crate::dataframe::DataFrame;
use crate::objective::Objective;
use crate::parameter::Parameter;
use crate::set::Set;
use crate::variable::Variable;
extern crate libc;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem::MaybeUninit;

/// A running AMPL interpreter instance.
///
/// All interaction with the AMPL engine goes through this struct: loading
/// models and data, solving, reading results, and setting data via DataFrames.
pub struct Ampl {
    pub(crate) raw: *mut ffi::AMPL,
}

impl Ampl {
    /// Create a new AMPL interpreter instance using the system PATH to locate the binary.
    pub fn new() -> Self {
        let mut ampl = MaybeUninit::uninit();
        let err = unsafe { ffi::AMPL_Create(ampl.as_mut_ptr()) };
        unsafe { check_ampl_error(err) };
        let ampl = unsafe { ampl.assume_init() };
        Ampl { raw: ampl }
    }

    /// Return a shallow copy sharing the same underlying AMPL pointer.
    pub fn clone(&self) -> Self {
        Ampl {
            raw: self.raw,
        }
    }

    /// Evaluate an arbitrary AMPL statement or expression.
    pub fn eval(&mut self, statement: &str) {
        let statement = CString::new(statement).unwrap();
        let err = unsafe { ffi::AMPL_Eval(self.raw, statement.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Solve the current problem, optionally specifying a sub-problem name and a solver.
    /// Pass empty strings to use the defaults already set in the model/options.
    pub fn solve(&self, problem: &str, solver: &str) {
        let problem = CString::new(problem).unwrap();
        let solver = CString::new(solver).unwrap();
        let err = unsafe { ffi::AMPL_Solve(self.raw, problem.as_ptr(), solver.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Reset the AMPL interpreter to a clean state, discarding model and data.
    pub fn reset(&mut self) {
        let err = unsafe { ffi::AMPL_Reset(self.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Close the underlying AMPL process. The instance should not be used afterward.
    pub fn close(&mut self) {
        let err = unsafe { ffi::AMPL_Close(self.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Return `true` if the underlying AMPL process is running.
    pub fn is_running(&mut self) -> bool {
        let mut running: bool = false;
        let err = unsafe { ffi::AMPL_IsRunning(self.raw, &mut running as *mut bool) };
        unsafe { check_ampl_error(err) };
        running
    }

    /// Return `true` if AMPL is currently busy (e.g. solving asynchronously).
    pub fn is_busy(&mut self) -> bool {
        let mut busy: bool = false;
        let err = unsafe { ffi::AMPL_IsBusy(self.raw, &mut busy as *mut bool) };
        unsafe { check_ampl_error(err) };
        busy
    }

    /// Send an interrupt signal to the running AMPL process.
    pub fn interrupt(&mut self) {
        let err = unsafe { ffi::AMPL_Interrupt(self.raw) };
        unsafe { check_ampl_error(err) };
    }

    /// Write a snapshot of the current session to `filename`.
    ///
    /// `model`, `data`, and `options` control which parts of the session are included.
    /// Returns the snapshot as a string.
    pub fn snapshot(&mut self, filename: &str, model: bool, data: bool, options: bool) -> String {
        let filename = CString::new(filename).unwrap();
        let mut snapshot_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_Snapshot(self.raw, filename.as_ptr(), model, data, options, &mut snapshot_ptr);
            check_ampl_error(err);
            if snapshot_ptr.is_null() {
                return String::new();
            }
            let snapshot_str = CStr::from_ptr(snapshot_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut snapshot_ptr);
            snapshot_str
        }
    }

    /// Export the current model to `filename` and return it as a string.
    pub fn export_model(&mut self, filename: &str) -> String {
        let filename = CString::new(filename).unwrap();
        let mut model_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_ExportModel(self.raw, filename.as_ptr(), &mut model_ptr);
            check_ampl_error(err);
            if model_ptr.is_null() {
                return String::new();
            }
            let model_str = CStr::from_ptr(model_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut model_ptr);
            model_str
        }
    }

    /// Export the current data to `filename` and return it as a string.
    pub fn export_data(&mut self, filename: &str) -> String {
        let filename = CString::new(filename).unwrap();
        let mut data_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_ExportData(self.raw, filename.as_ptr(), &mut data_ptr);
            check_ampl_error(err);
            if data_ptr.is_null() {
                return String::new();
            }
            let data_str = CStr::from_ptr(data_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut data_ptr);
            data_str
        }
    }

    /// Return the name of the currently active objective, or an empty string if none is set.
    pub fn get_current_objective(&mut self) -> String {
        let mut objective_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_GetCurrentObjective(self.raw, &mut objective_ptr);
            check_ampl_error(err);
            if objective_ptr.is_null() {
                return String::new();
            }
            let objective_str = CStr::from_ptr(objective_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut objective_ptr);
            objective_str
        }
    }

    /// Set a string-valued AMPL option.
    pub fn set_option(&mut self, option: &str, value: &str) {
        let option = CString::new(option).unwrap();
        let value = CString::new(value).unwrap();
        let err = unsafe { ffi::AMPL_SetOption(self.raw, option.as_ptr(), value.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Set a boolean AMPL option.
    pub fn set_bool_option(&mut self, option: &str, value: bool) {
        let option = CString::new(option).unwrap();
        let err = unsafe { ffi::AMPL_SetBoolOption(self.raw, option.as_ptr(), value) };
        unsafe { check_ampl_error(err) };
    }

    /// Set an integer AMPL option.
    pub fn set_int_option(&mut self, option: &str, value: i32) {
        let option = CString::new(option).unwrap();
        let err = unsafe { ffi::AMPL_SetIntOption(self.raw, option.as_ptr(), value) };
        unsafe { check_ampl_error(err) };
    }

    /// Set a double AMPL option.
    pub fn set_dbl_option(&mut self, option: &str, value: f64) {
        let option = CString::new(option).unwrap();
        let err = unsafe { ffi::AMPL_SetDblOption(self.raw, option.as_ptr(), value) };
        unsafe { check_ampl_error(err) };
    }

    /// Get the string value of an AMPL option. Returns an empty string if the option does not exist.
    pub fn get_option(&mut self, option: &str) -> String {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value_ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_GetOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value_ptr);
            check_ampl_error(err);
            if value_ptr.is_null() {
                return String::new();
            }
            let value_str = CStr::from_ptr(value_ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut value_ptr);
            value_str
        }
    }

    /// Get the boolean value of an AMPL option. Returns `false` if the option does not exist.
    pub fn get_bool_option(&mut self, option: &str) -> bool {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: bool = false;
        let err = unsafe { ffi::AMPL_GetBoolOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut bool) };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Get the integer value of an AMPL option. Returns `0` if the option does not exist.
    pub fn get_int_option(&mut self, option: &str) -> i32 {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: i32 = 0;
        let err = unsafe { ffi::AMPL_GetIntOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut i32) };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Get the double value of an AMPL option. Returns `0.0` if the option does not exist.
    pub fn get_dbl_option(&mut self, option: &str) -> f64 {
        let option = CString::new(option).unwrap();
        let mut exists: bool = false;
        let mut value: f64 = 0.0;
        let err = unsafe { ffi::AMPL_GetDblOption(self.raw, option.as_ptr(), &mut exists as *mut bool, &mut value as *mut f64) };
        unsafe { check_ampl_error(err) };
        value
    }

    /// Read and execute an AMPL model file at `filename`.
    pub fn read(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        let err = unsafe { ffi::AMPL_Read(self.raw, filename.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Read an AMPL data file at `filename`.
    pub fn read_data(&mut self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        let err = unsafe { ffi::AMPL_ReadData(self.raw, filename.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Read a table named `tablename` into AMPL (equivalent to `read table tablename;`).
    pub fn read_table(&mut self, tablename: &str) {
        let tablename = CString::new(tablename).unwrap();
        let err = unsafe { ffi::AMPL_ReadTable(self.raw, tablename.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Write a table named `tablename` from AMPL (equivalent to `write table tablename;`).
    pub fn write_table(&mut self, tablename: &str) {
        let tablename = CString::new(tablename).unwrap();
        let err = unsafe { ffi::AMPL_WriteTable(self.raw, tablename.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Write the model to `filename` with auxiliary files listed in `auxfiles`.
    pub fn write(&mut self, filename: &str, auxfiles: &str) {
        let filename = CString::new(filename).unwrap();
        let auxfiles = CString::new(auxfiles).unwrap();
        let err = unsafe { ffi::AMPL_Write(self.raw, filename.as_ptr(), auxfiles.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Assign data from `df` to the AMPL entities whose names match the DataFrame's column headers.
    ///
    /// If `set_name` is `Some("S")`, the index column values are also assigned to set `S`.
    pub fn set_data(&mut self, df: &DataFrame, set_name: Option<&str>) {
        let set_name_c = CString::new(set_name.unwrap_or("")).unwrap();
        let set_name_ptr = set_name_c.as_ptr();
        let err = unsafe { ffi::AMPL_SetData(self.raw, df.raw, set_name_ptr) };
        unsafe { check_ampl_error(err) };
    }

    /// Retrieve data from AMPL for the given display `statements` and return it as a DataFrame.
    ///
    /// `statements` may be parameter/variable names or arbitrary AMPL expressions.
    /// All statements must be indexable over the same set.
    pub fn get_data(&mut self, statements: &[&str]) -> DataFrame {
        let cstrings: Vec<CString> = statements.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let ptrs: Vec<*const libc::c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetData(self.raw, ptrs.as_ptr(), statements.len(), &mut df) };
        unsafe { check_ampl_error(err) };
        DataFrame { raw: df }
    }

    /// Return the constraint with the given AMPL name.
    pub fn get_constraint(&mut self, name: &str) -> Constraint {
        Constraint::new(self, name.to_string())
    }

    /// Return all constraints declared in the current model.
    pub fn get_constraints(&mut self) -> Vec<Constraint> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetConstraints(self.raw, &mut size, &mut names) };
        unsafe { check_ampl_error(err) };

        let mut constraints = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                constraints.push(Constraint::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            libc::free(names as *mut libc::c_void);
        }
        constraints
    }

    /// Return the objective with the given AMPL name.
    pub fn get_objective(&mut self, name: &str) -> Objective {
        Objective {raw: self.raw, name: name.to_string()}
    }

    /// Return all objectives declared in the current model.
    pub fn get_objectives(&mut self) -> Vec<Objective> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetObjectives(self.raw, &mut size, &mut names) };
        unsafe { check_ampl_error(err) };

        let mut objectives = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                objectives.push(Objective {raw: self.raw, name: name_string});
                ffi::AMPL_StringFree(names.add(i));
            }
            libc::free(names as *mut libc::c_void);
        }
        objectives
    }

    /// Return the parameter with the given AMPL name.
    pub fn get_parameter(&mut self, name: &str) -> Parameter {
        Parameter::new(self, name.to_string())
    }

    /// Return all parameters declared in the current model.
    pub fn get_parameters(&mut self) -> Vec<Parameter> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetParameters(self.raw, &mut size, &mut names) };
        unsafe { check_ampl_error(err) };

        let mut parameters = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                parameters.push(Parameter::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            libc::free(names as *mut libc::c_void);
        }
        parameters
    }

    /// Return the set with the given AMPL name.
    pub fn get_set(&mut self, name: &str) -> Set {
        Set::new(self, name.to_string())
    }

    /// Return all sets declared in the current model.
    pub fn get_sets(&mut self) -> Vec<Set> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetSets(self.raw, &mut size, &mut names) };
        unsafe { check_ampl_error(err) };

        let mut sets = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                sets.push(Set::new(self, name_string));
                ffi::AMPL_StringFree(names.add(i));
            }
            libc::free(names as *mut libc::c_void);
        }
        sets
    }

    /// Return the variable with the given AMPL name.
    pub fn get_variable(&mut self, name: &str) -> Variable {
        Variable {ampl: self, name: name.to_string()}
    }

    /// Return all variables declared in the current model.
    pub fn get_variables(&mut self) -> Vec<Variable> {
        let mut size: usize = 0;
        let mut names: *mut *mut c_char = ptr::null_mut();
        let err = unsafe { ffi::AMPL_GetVariables(self.raw, &mut size, &mut names) };
        unsafe { check_ampl_error(err) };

        let mut variables = Vec::with_capacity(size);
        unsafe {
            for i in 0..size {
                let name_ptr = *names.add(i);
                let name_string = CStr::from_ptr(name_ptr).to_str().unwrap().to_string();
                variables.push(Variable {ampl: self, name: name_string});
                ffi::AMPL_StringFree(names.add(i));
            }
            libc::free(names as *mut libc::c_void);
        }
        variables
    }
}

impl Drop for Ampl {
    fn drop(&mut self) {
        unsafe { ffi::AMPL_Free(&mut self.raw) };
    }
}
