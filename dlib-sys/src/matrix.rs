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

    pub fn from_opencv(mat: &mut Mat) -> Self {
        let mut inner = crate::ffi::wrapper::Matrix::new().within_unique_ptr();

        inner.pin_mut().fromOpenCV(mat.inner.pin_mut());

        Self { inner }
    }
}
