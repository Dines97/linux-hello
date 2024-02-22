use autocxx::prelude::*;

pub(super) struct Mat {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::cv::Mat>,
}

impl Mat {
    pub(super) fn new() -> Self {
        Self {
            inner: crate::ffi::cv::Mat::new().within_unique_ptr(),
        }
    }
}
