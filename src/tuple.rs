use crate::ffi;

pub struct Tuple {
    pub raw: *mut ffi::AMPL_TUPLE,
}

impl Tuple {
    pub fn new(raw: *mut ffi::AMPL_TUPLE) -> Self {
        Tuple { raw: raw }
    }
}