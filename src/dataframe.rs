use std::mem::MaybeUninit;

/// Represents of AMPL.
pub struct Dataframe {
    pub(crate) raw: *mut ffi::AMPL_DATAFRAME,
}


impl Dataframe {
    pub fn new(numIndexColumns: &usize, numDataColumns: &usize, headers: &[&str]) -> Self {
        let mut df = MaybeUninit::uninit();
        // Step 1: Convert each &str to a CString
        let cstrings: Vec<CString> = headers.iter()
        .map(|&s| CString::new(s).unwrap())
        .collect();
        // Step 2: Collect pointers to the internal buffer of each CString
        let mut c_char_ptrs: Vec<*const c_char> = cstrings.iter()
        .map(|s| s.as_ptr())
        .collect();
        // Ensure null-termination for C compatibility
        c_char_ptrs.push(std::ptr::null());
        // Step 3: Convert the Vec<*const c_char> to a raw pointer
        let ptr = c_char_ptrs.as_ptr();

        unsafe { ffi::AMPL_DataFrameCreate(df.as_mut_ptr(), numIndexColumns, numDataColumns, ptr) };
        let df = unsafe { df.assume_init() };
        Dataframe { raw: df }
    }
}