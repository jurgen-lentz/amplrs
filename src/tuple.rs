use crate::ffi;

/// A tuple of AMPL variants used to index into multi-dimensional entities.
///
/// Tuples are produced by the AMPL engine (e.g. when iterating instances) and
/// are passed back to API calls that require an instance index.
pub struct Tuple {
    pub raw: *mut ffi::AMPL_TUPLE,
}

impl Tuple {
    /// Wrap an existing raw AMPL tuple pointer. The caller retains ownership.
    pub fn new(raw: *mut ffi::AMPL_TUPLE) -> Self {
        Tuple { raw: raw }
    }
}
