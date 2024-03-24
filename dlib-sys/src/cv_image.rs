use autocxx::prelude::*;
use opencv_sys::mat::Mat;

pub struct CvImage {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::CvImage>,
}

impl CvImage {
    pub fn new(mat: &Mat) -> Self {
        Self {
            inner: crate::ffi::wrapper::CvImage::new(mat.inner.as_ref().unwrap())
                .within_unique_ptr(),
        }
    }
}
