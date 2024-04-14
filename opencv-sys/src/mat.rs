use autocxx::prelude::*;

pub struct Mat {
    pub inner: cxx::UniquePtr<crate::ffi::cv::Mat>,
}

impl Default for Mat {
    fn default() -> Self {
        Self {
            inner: crate::ffi::cv::Mat::new().within_unique_ptr(),
        }
    }
}

unsafe impl Send for Mat {}
