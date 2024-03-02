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

    pub fn assing_image(&mut self, mat: &mut Mat) {
        self.inner.pin_mut().assignImage(mat.inner.pin_mut());
    }
}
