use crate::error::check_ampl_error;
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

/// Build an `AMPL_TUPLE` from a slice of [`Value`]s.
///
/// The caller is responsible for freeing the returned tuple with
/// `AMPL_TupleFree`. CStrings are kept alive until after tuple creation so
/// that string pointers remain valid.
unsafe fn values_to_tuple(values: &[Value]) -> *mut ffi::AMPL_TUPLE {
    let cstrings: Vec<Option<CString>> = values.iter().map(|v| match v {
        Value::Numeric(_) => None,
        Value::Text(s) => Some(CString::new(s.as_str()).unwrap()),
    }).collect();

    let mut variant_ptrs: Vec<*mut ffi::AMPL_VARIANT> = Vec::with_capacity(values.len());
    for (v, cs) in values.iter().zip(cstrings.iter()) {
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            match v {
                Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                Value::Text(_)    => { ffi::AMPL_VariantCreateString(&mut var, cs.as_ref().unwrap().as_ptr()); }
            }
        }
        variant_ptrs.push(var);
    }

    let mut tuple: *mut ffi::AMPL_TUPLE = ptr::null_mut();
    unsafe { ffi::AMPL_TupleCreate(&mut tuple, variant_ptrs.len(), variant_ptrs.as_mut_ptr()); }
    for var in &mut variant_ptrs {
        unsafe { ffi::AMPL_VariantFree(var); }
    }
    tuple
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
        let mut c_ptrs: Vec<*const c_char> = cstrings.iter()
            .map(|s| s.as_ptr())
            .collect();
        c_ptrs.push(ptr::null()); // AMPL_DataFrameCreate reads until null
        let mut df: *mut ffi::AMPL_DATAFRAME = ptr::null_mut();
        let err = unsafe {
            ffi::AMPL_DataFrameCreate(&mut df, num_index_cols, num_data_cols, c_ptrs.as_ptr())
        };
        unsafe { check_ampl_error(err) };
        DataFrame { raw: df }
    }

    /// Return the number of data rows.
    pub fn num_rows(&self) -> usize {
        let mut n: usize = 0;
        let err = unsafe { ffi::AMPL_DataFrameGetNumRows(self.raw, &mut n) };
        unsafe { check_ampl_error(err) };
        n
    }

    /// Return the total number of columns (index columns + data columns).
    pub fn num_cols(&self) -> usize {
        let mut n: usize = 0;
        let err = unsafe { ffi::AMPL_DataFrameGetNumCols(self.raw, &mut n) };
        unsafe { check_ampl_error(err) };
        n
    }

    /// Return the number of index columns.
    pub fn num_indices(&self) -> usize {
        let mut n: usize = 0;
        let err = unsafe { ffi::AMPL_DataFrameGetNumIndices(self.raw, &mut n) };
        unsafe { check_ampl_error(err) };
        n
    }

    /// Return the column headers in order (index columns first, then data columns).
    pub fn headers(&self) -> Vec<String> {
        let mut size: usize = 0;
        let mut headers_ptr: *mut *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_DataFrameGetHeaders(self.raw, &mut size, &mut headers_ptr);
            check_ampl_error(err);
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
        let err = unsafe { ffi::AMPL_DataFrameReserve(self.raw, n) };
        unsafe { check_ampl_error(err) };
    }

    /// Append a row of mixed-type values. The number of values must equal `num_cols()`.
    pub fn add_row(&self, values: &[Value]) {
        // Collect CStrings first so their data stays alive until after TupleCreate.
        let cstrings: Vec<Option<CString>> = values.iter().map(|v| match v {
            Value::Numeric(_) => None,
            Value::Text(s) => Some(CString::new(s.as_str()).unwrap()),
        }).collect();

        let mut variant_ptrs: Vec<*mut ffi::AMPL_VARIANT> = Vec::with_capacity(values.len());
        for (v, cs) in values.iter().zip(cstrings.iter()) {
            let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
            unsafe {
                match v {
                    Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                    Value::Text(_)    => { ffi::AMPL_VariantCreateString(&mut var, cs.as_ref().unwrap().as_ptr()); }
                }
            }
            variant_ptrs.push(var);
        }

        let mut tuple: *mut ffi::AMPL_TUPLE = ptr::null_mut();
        unsafe {
            ffi::AMPL_TupleCreate(&mut tuple, variant_ptrs.len(), variant_ptrs.as_mut_ptr());
            let err = ffi::AMPL_DataFrameAddRow(self.raw, tuple);
            check_ampl_error(err);
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

    /// Append a new empty data column with the given `header`.
    pub fn add_empty_column(&self, header: &str) {
        let header_c = CString::new(header).unwrap();
        let err = unsafe { ffi::AMPL_DataFrameAddEmptyColumn(self.raw, header_c.as_ptr()) };
        unsafe { check_ampl_error(err) };
    }

    /// Append a new numeric data column named `header` with the given values.
    ///
    /// The number of rows must already be established (e.g. by a prior `set_column_*` call).
    /// The length of `values` must equal the current row count.
    pub fn add_column_doubles(&self, header: &str, values: &[f64]) {
        let header_c = CString::new(header).unwrap();
        let err = unsafe {
            ffi::AMPL_DataFrameAddColumnDouble(self.raw, header_c.as_ptr(), values.as_ptr())
        };
        unsafe { check_ampl_error(err) };
    }

    /// Append a new string data column named `header` with the given values.
    ///
    /// The number of rows must already be established. The length of `values` must equal
    /// the current row count.
    pub fn add_column_strings(&self, header: &str, values: &[&str]) {
        let header_c = CString::new(header).unwrap();
        let cstrings: Vec<CString> = values.iter().map(|&s| CString::new(s).unwrap()).collect();
        let mut ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let err = unsafe {
            ffi::AMPL_DataFrameAddColumnString(self.raw, header_c.as_ptr(), ptrs.as_mut_ptr())
        };
        unsafe { check_ampl_error(err) };
    }

    /// Overwrite an entire numeric column identified by `header` with the given values.
    pub fn set_column_doubles(&self, header: &str, values: &[f64]) {
        let header_c = CString::new(header).unwrap();
        let err = unsafe {
            ffi::AMPL_DataFrameSetColumnArgDouble(
                self.raw,
                header_c.as_ptr(),
                values.as_ptr(),
                values.len(),
            )
        };
        unsafe { check_ampl_error(err) };
    }

    /// Overwrite an entire string column identified by `header` with the given values.
    pub fn set_column_strings(&self, header: &str, values: &[&str]) {
        let header_c = CString::new(header).unwrap();
        let cstrings: Vec<CString> = values.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let err = unsafe {
            ffi::AMPL_DataFrameSetColumnArgString(
                self.raw,
                header_c.as_ptr(),
                ptrs.as_ptr(),
                values.len(),
            )
        };
        unsafe { check_ampl_error(err) };
    }

    /// Return all values in the column named `header`, one entry per row.
    pub fn get_column(&self, header: &str) -> Vec<Value> {
        let header_c = CString::new(header).unwrap();
        let mut col_idx: usize = 0;
        let err = unsafe {
            ffi::AMPL_DataFrameGetColumnIndex(self.raw, header_c.as_ptr(), &mut col_idx)
        };
        unsafe { check_ampl_error(err) };
        let nrows = self.num_rows();
        (0..nrows).map(|row| self.get_value(row, col_idx)).collect()
    }

    /// Set the value at the given 0-based `(row, col)` position.
    pub fn set_value(&self, row: usize, col: usize, value: &Value) {
        let cs = match value {
            Value::Text(s) => Some(CString::new(s.as_str()).unwrap()),
            _ => None,
        };
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            match value {
                Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                Value::Text(_)    => { ffi::AMPL_VariantCreateString(&mut var, cs.as_ref().unwrap().as_ptr()); }
            }
            let err = ffi::AMPL_DataFrameSetValueByIndex(self.raw, row, col, var);
            check_ampl_error(err);
            ffi::AMPL_VariantFree(&mut var);
        }
    }

    /// Set the value in the column named `header` at the row identified by the index `key`.
    ///
    /// `key` must contain one [`Value`] per index column.
    pub fn set_value_at(&self, key: &[Value], header: &str, value: &Value) {
        let header_c = CString::new(header).unwrap();
        let cs = match value {
            Value::Text(s) => Some(CString::new(s.as_str()).unwrap()),
            _ => None,
        };
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            match value {
                Value::Numeric(n) => { ffi::AMPL_VariantCreateNumeric(&mut var, *n); }
                Value::Text(_)    => { ffi::AMPL_VariantCreateString(&mut var, cs.as_ref().unwrap().as_ptr()); }
            }
            let mut tuple = values_to_tuple(key);
            let err = ffi::AMPL_DataFrameSetValue(self.raw, tuple, header_c.as_ptr(), var);
            check_ampl_error(err);
            ffi::AMPL_TupleFree(&mut tuple);
            ffi::AMPL_VariantFree(&mut var);
        }
    }

    /// Get the value at the given 0-based `(row, col)` position.
    pub fn get_value(&self, row: usize, col: usize) -> Value {
        let mut var: *mut ffi::AMPL_VARIANT = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_DataFrameElement(self.raw, row, col, &mut var);
            check_ampl_error(err);
            let mut type_: ffi::AMPL_TYPE = ffi::AMPL_TYPE_AMPL_NUMERIC;
            ffi::AMPL_VariantGetType(var, &mut type_);
            if type_ == ffi::AMPL_TYPE_AMPL_STRING {
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
            }
        }
    }

    /// Return all values in row `index` as a `Vec<Value>` (all columns, left to right).
    pub fn get_row_by_index(&self, index: usize) -> Vec<Value> {
        let ncols = self.num_cols();
        (0..ncols).map(|col| self.get_value(index, col)).collect()
    }

    /// Find the 0-based row position for the index tuple `key`.
    ///
    /// Returns `None` if no row with that key exists.
    pub fn find_row(&self, key: &[Value]) -> Option<usize> {
        let mut row_idx: usize = 0;
        unsafe {
            let mut tuple = values_to_tuple(key);
            let err = ffi::AMPL_DataFrameGetRowIndex(self.raw, tuple, &mut row_idx);
            check_ampl_error(err);
            ffi::AMPL_TupleFree(&mut tuple);
        }
        let nrows = self.num_rows();
        if row_idx == nrows { None } else { Some(row_idx) }
    }

    /// Return all values in the row identified by the index tuple `key`.
    ///
    /// Panics if no row with that key exists.
    pub fn get_row(&self, key: &[Value]) -> Vec<Value> {
        let idx = self.find_row(key).expect("no row with the given key");
        self.get_row_by_index(idx)
    }

    /// Return an iterator over all rows. Each item is a `Vec<Value>` of all column values.
    pub fn rows(&self) -> impl Iterator<Item = Vec<Value>> + '_ {
        let nrows = self.num_rows();
        let ncols = self.num_cols();
        (0..nrows).map(move |row| {
            (0..ncols).map(|col| self.get_value(row, col)).collect()
        })
    }

    /// Fill a 1-index / 1-data-column DataFrame from parallel slices of string indices and `f64` values.
    pub fn set_array(&self, indices: &[&str], values: &[f64]) {
        let cstrings: Vec<CString> = indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let ptrs: Vec<*const c_char> = cstrings.iter().map(|s| s.as_ptr()).collect();
        let mut args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        unsafe {
            ffi::AMPL_ArgsCreateString(&mut args, ptrs.as_ptr());
            let err = ffi::AMPL_DataFrameSetArray(self.raw, values.as_ptr(), values.len(), args);
            check_ampl_error(err);
            ffi::AMPL_ArgsDestroy(&mut args);
        }
    }

    /// Fill a 2-index / 1-data-column DataFrame from parallel slices of string row indices,
    /// string column indices, and a flat row-major `f64` values array.
    ///
    /// `values.len()` must equal `row_indices.len() * col_indices.len()`.
    pub fn set_matrix_doubles(&self, row_indices: &[&str], col_indices: &[&str], values: &[f64]) {
        let row_cs: Vec<CString> = row_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let row_ptrs: Vec<*const c_char> = row_cs.iter().map(|s| s.as_ptr()).collect();
        let col_cs: Vec<CString> = col_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let col_ptrs: Vec<*const c_char> = col_cs.iter().map(|s| s.as_ptr()).collect();
        let mut row_args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        let mut col_args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        unsafe {
            ffi::AMPL_ArgsCreateString(&mut row_args, row_ptrs.as_ptr());
            ffi::AMPL_ArgsCreateString(&mut col_args, col_ptrs.as_ptr());
            let err = ffi::AMPL_DataFrameSetMatrix(
                self.raw,
                values.as_ptr(),
                row_indices.len(),
                row_args,
                col_indices.len(),
                col_args,
            );
            check_ampl_error(err);
            ffi::AMPL_ArgsDestroy(&mut row_args);
            ffi::AMPL_ArgsDestroy(&mut col_args);
        }
    }

    /// Fill a 2-index / 1-data-column DataFrame from parallel slices of string row indices,
    /// string column indices, and a flat row-major string values array.
    ///
    /// `values.len()` must equal `row_indices.len() * col_indices.len()`.
    pub fn set_matrix_strings(&self, row_indices: &[&str], col_indices: &[&str], values: &[&str]) {
        let row_cs: Vec<CString> = row_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let mut row_ptrs: Vec<*const c_char> = row_cs.iter().map(|s| s.as_ptr()).collect();
        row_ptrs.push(ptr::null());
        let col_cs: Vec<CString> = col_indices.iter().map(|&s| CString::new(s).unwrap()).collect();
        let mut col_ptrs: Vec<*const c_char> = col_cs.iter().map(|s| s.as_ptr()).collect();
        col_ptrs.push(ptr::null());
        let val_cs: Vec<CString> = values.iter().map(|&s| CString::new(s).unwrap()).collect();
        let val_ptrs: Vec<*const c_char> = val_cs.iter().map(|s| s.as_ptr()).collect();
        let mut row_args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        let mut col_args: *mut ffi::AMPL_ARGS = ptr::null_mut();
        unsafe {
            ffi::AMPL_ArgsCreateString(&mut row_args, row_ptrs.as_ptr());
            ffi::AMPL_ArgsCreateString(&mut col_args, col_ptrs.as_ptr());
            let err = ffi::AMPL_DataFrameSetMatrixString(
                self.raw,
                val_ptrs.as_ptr(),
                row_indices.len(),
                row_args,
                col_indices.len(),
                col_args,
            );
            check_ampl_error(err);
            ffi::AMPL_ArgsDestroy(&mut row_args);
            ffi::AMPL_ArgsDestroy(&mut col_args);
        }
    }

    /// Return a human-readable tabular string representation of the DataFrame.
    pub fn to_string(&self) -> String {
        let mut ptr: *mut c_char = ptr::null_mut();
        unsafe {
            let err = ffi::AMPL_DataFrameToString(self.raw, &mut ptr);
            check_ampl_error(err);
            if ptr.is_null() {
                return String::new();
            }
            let s = CStr::from_ptr(ptr).to_str().unwrap().to_string();
            ffi::AMPL_StringFree(&mut ptr);
            s
        }
    }
}

impl PartialEq for DataFrame {
    /// Return `true` if both DataFrames have identical structure and contents.
    fn eq(&self, other: &Self) -> bool {
        let mut equals: std::ffi::c_int = 0;
        let err = unsafe { ffi::AMPL_DataFrameEquals(self.raw, other.raw, &mut equals) };
        unsafe { check_ampl_error(err) };
        equals != 0
    }
}

impl Drop for DataFrame {
    fn drop(&mut self) {
        unsafe { ffi::AMPL_DataFrameFree(&mut self.raw) };
    }
}
