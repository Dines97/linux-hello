use autocxx::prelude::*;

pub struct Mat {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::cv::Mat>,
}

impl Mat {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::cv::Mat::new().within_unique_ptr(),
        }
    }
}
