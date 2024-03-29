use autocxx::prelude::*;
use opencv_sys::mat::Mat;

pub struct Matrix {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::Matrix>,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::Matrix::new().within_unique_ptr(),
        }
    }
}
