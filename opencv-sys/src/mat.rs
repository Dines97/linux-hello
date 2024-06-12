use autocxx::prelude::*;
use std::pin::Pin;

pub struct Mat {
    pub inner: Pin<Box<crate::ffi::cv::Mat>>,
}

impl Default for Mat {
    fn default() -> Self {
        Self {
            inner: crate::ffi::cv::Mat::new().within_box(),
        }
    }
}

unsafe impl Send for Mat {}
