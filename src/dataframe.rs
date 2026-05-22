use crate::ffi;

use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

/// A value stored in an AMPL DataFrame cell.
pub enum Value {
    /// A numeric (floating-point) value.
    Numeric(f64),
    /// A string value.
    Text(String),
}

impl Value {
    /// Return the numeric value, or `None` if this is a string variant.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Numeric(v) => Some(*v),
            Value::Text(_) => None,
        }
    }

    /// Return the string value, or `None` if this is a numeric variant.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Numeric(_) => None,
            Value::Text(s) => Some(s.as_str()),
        }
    }
}

/// An AMPL DataFrame for passing tabular data between Rust and AMPL.
///
/// A DataFrame has zero or more *index* columns (used to key rows) followed by
/// zero or more *data* columns.  The total column count is `num_indices() + num_data_cols`.
pub struct DataFrame {
    pub(crate) raw: *mut ffi::AMPL_DATAFRAME,
}

impl DataFrame {
    /// Create a new DataFrame with `num_index_cols` index columns and `num_data_cols` data columns.
    ///
    /// `headers` must contain exactly `num_index_cols + num_data_cols` entries, in order.
    pub fn new(num_index_cols: usize, num_data_cols: usize, headers: &[&str]) -> Self {
        let cstrings: Vec<CString> = headers.iter()
            .map(|&s| CString::new(s).unwrap())
            .collect();
        let c_ptrs: Vec<*const c_char> = cstrings.iter()
            .map(|s| s.as_ptr())
            .collect();
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        unsafe {
            ffi::AMPL_DataFrameCreate(&mut df, num_index_cols, num_data_cols, c_ptrs.as_ptr());
        }
        DataFrame { raw: df }
    }

    /// Return the number of data rows.
    pub fn num_rows(&self) -> usize {
        let mut n: usize = 0;
        unsafe { ffi::AMPL_DataFrameGetNumRows(self.raw, &mut n) };
        n
    }

    /// Return the total number of columns (index columns + data columns).
    pub fn num_cols(&self) -> usize {
        let mut n: usize = 0;
        unsafe { ffi::AMPL_DataFrameGetNumCols(self.raw, &mut n) };
        n
    }

    /// Return the number of index columns.
    pub fn num_indices(&self) -> usize {
        let mut n: usize = 0;
        unsafe { ffi::AMPL_DataFrameGetNumIndices(self.raw, &mut n) };
        n
    }

    /// Return the column headers in order (index columns first, then data columns).
    pub fn headers(&self) -> Vec<String> {
        let mut size: usize = 0;
        let mut headers_ptr: *mut *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_DataFrameGetHeaders(self.raw, &mut size, &mut headers_ptr);
            let mut result = Vec::with_capacity(size);
            for i in 0..size {
                let s = CStr::from_ptr(*headers_ptr.add(i)).to_str().unwrap().to_string();
                result.push(s);
                ffi::AMPL_StringFree(headers_ptr.add(i));
            }
            libc::free(headers_ptr as *mut libc::c_void);
            result
        }
    }

    /// Pre-allocate space for `n` rows. Rows still must be added one by one via [`add_row`].
    pub fn reserve(&self, n: usize) {
        unsafe { ffi::AMPL_DataFrameReserve(self.raw, n) };
    }

    /// Append a row of mixed-type values. The number of values must equal `num_cols()`.
    pub fn add_row(&self, values: &[Value]) {
        let mut variant_ptrs: Vec<*mut ffi::AMPL_VARIANT> = values.iter().map(|v| {
            let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
            unsafe {
                match v {
                    Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                    Value::Text(s) => {
                        let cs = CString::new(s.as_str()).unwrap();
                        ffi::AMPL_VariantCreateString(&mut var, cs.as_ptr());
                    }
                }
            }
            var
        }).collect();

        let mut tuple: *mut ffi::AMPL_TUPLE = ptr::null_mut();
        unsafe {
            ffi::AMPL_TupleCreate(&mut tuple, variant_ptrs.len(), variant_ptrs.as_mut_ptr());
            ffi::AMPL_DataFrameAddRow(self.raw, tuple);
            ffi::AMPL_TupleFree(&mut tuple);
            for var in &mut variant_ptrs {
                ffi::AMPL_VariantFree(var);
            }
        }
    }

    /// Append a row of all-numeric values. The length must equal `num_cols()`.
    pub fn add_row_doubles(&self, values: &[f64]) {
        let row: Vec<Value> = values.iter().map(|&v| Value::Numeric(v)).collect();
        self.add_row(&row);
    }

    /// Append a row of all-string values. The length must equal `num_cols()`.
    pub fn add_row_strings(&self, values: &[&str]) {
        let row: Vec<Value> = values.iter().map(|&v| Value::Text(v.to_string())).collect();
        self.add_row(&row);
    }

    /// Append a new numeric data column named `header` with the given values.
    ///
    /// The number of rows must already be established (e.g. by a prior `set_column_*` call).
    /// The length of `values` must equal the current row count.
    pub fn add_column_doubles(&self, header: &str, values: &[f64]) {
        let header_c = CString::new(header).unwrap();
        unsafe {
            ffi::AMPL_DataFrameAddColumnDouble(self.raw, header_c.as_ptr(), values.as_ptr());
        }
    }

    /// Append a new string data column named `header` with the given values.
    ///
    /// The number of rows must already be established. The length of `values` must equal
    /// the current row count.
    pub fn add_column_strings(&self, header: &str, values: &[&str]) {
        let header_c = CString::new(header).unwrap();
        let cstrings: Vec<CString> = values.iter().map(|&s| CString::new(s).unwrap()).collect();
        let mut ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        unsafe {
            ffi::AMPL_DataFrameAddColumnString(self.raw, header_c.as_ptr(), ptrs.as_mut_ptr());
        }
    }

    /// Overwrite an entire numeric column identified by `header` with the given values.
    pub fn set_column_doubles(&self, header: &str, values: &[f64]) {
        let header_c = CString::new(header).unwrap();
        unsafe {
            ffi::AMPL_DataFrameSetColumnArgDouble(
                self.raw,
                header_c.as_ptr(),
                values.as_ptr(),
                values.len(),
            );
        }
    }

    /// Overwrite an entire string column identified by `header` with the given values.
    pub fn set_column_strings(&self, header: &str, values: &[&str]) {
        let header_c = CString::new(header).unwrap();
        let cstrings: Vec<CString> = values.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        unsafe {
            ffi::AMPL_DataFrameSetColumnArgString(
                self.raw,
                header_c.as_ptr(),
                ptrs.as_ptr(),
                values.len(),
            );
        }
    }

    /// Set the value at the given 0-based `(row, col)` position.
    pub fn set_value(&self, row: usize, col: usize, value: &Value) {
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            match value {
                Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                Value::Text(s) => {
                    let cs = CString::new(s.as_str()).unwrap();
                    ffi::AMPL_VariantCreateString(&mut var, cs.as_ptr());
                }
            }
            ffi::AMPL_DataFrameSetValueByIndex(self.raw, row, col, var);
            ffi::AMPL_VariantFree(&mut var);
        }
    }

    /// Get the value at the given 0-based `(row, col)` position.
    pub fn get_value(&self, row: usize, col: usize) -> Value {
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            ffi::AMPL_DataFrameElement(self.raw, row, col, &mut var);
            let mut type_: ffi::AMPL_TYPE = ffi::AMPL_TYPE_AMPL_NUMERIC;
            ffi::AMPL_VariantGetType(var, &mut type_);
            let result = if type_ == ffi::AMPL_TYPE_AMPL_STRING {
                let mut s_ptr: *mut c_char = ptr::null_mut();
                ffi::AMPL_VariantGetStringValue(var, &mut s_ptr);
                let s = if s_ptr.is_null() {
                    String::new()
                } else {
                    let owned = CStr::from_ptr(s_ptr).to_str().unwrap().to_string();
                    ffi::AMPL_StringFree(&mut s_ptr);
                    owned
                };
                Value::Text(s)
            } else {
                let mut v: f64 = 0.0;
                ffi::AMPL_VariantGetNumericValue(var, &mut v);
                Value::Numeric(v)
            };
            ffi::AMPL_VariantFree(&mut var);
            result
        }
    }

    /// Fill a 1-index / 1-data-column DataFrame from parallel slices of string indices and `f64` values.
    pub fn set_array(&self, indices: &[&str], values: &[f64]) {
        let cstrings: Vec<CString> = indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let mut args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        unsafe {
            ffi::AMPL_ArgsCreateString(&mut args, ptrs.as_ptr());
            ffi::AMPL_DataFrameSetArray(self.raw, values.as_ptr(), values.len(), args);
            ffi::AMPL_ArgsDestroy(&mut args);
        }
    }

    /// Fill a 2-index / 1-data-column DataFrame from row indices, column indices, and a flat
    /// row-major matrix of `row_indices.len() × col_indices.len()` values.
    pub fn set_matrix(&self, row_indices: &[&str], col_indices: &[&str], values: &[f64]) {
        let row_cs: Vec<CString> = row_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let row_ptrs: Vec<*const c_char> = row_cs.iter().map(|s| s.as_ptr()).collect();
        let col_cs: Vec<CString> = col_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let col_ptrs: Vec<*const c_char> = col_cs.iter().map(|s| s.as_ptr()).collect();
        unsafe {
            ffi::AMPL_DataFrameSetMatrixStringString(
                self.raw,
                values.as_ptr(),
                row_indices.len(),
                row_ptrs.as_ptr(),
                col_indices.len(),
                col_ptrs.as_ptr(),
            );
        }
    }

    /// Return a human-readable tabular string representation of the DataFrame.
    pub fn to_string(&self) -> String {
        let mut ptr: *mut c_char = ptr::null_mut();
        unsafe {
            ffi::AMPL_DataFrameToString(self.raw, &mut ptr);
            if ptr.is_null() {
                return String::new();
            }
            let s = CStr::from_ptr(ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut ptr);
            s
        }
    }
}

impl Drop for DataFrame {
    fn drop(&mut self) {
        unsafe { ffi::AMPL_DataFrameFree(&mut self.raw) };
    }
}
