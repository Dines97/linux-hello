use autocxx::prelude::*;
use serde::{Deserialize, Serialize};

use crate::mat::Mat;

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum VideoCaptureAPIs {
    CapAny = crate::ffi::cv::VideoCaptureAPIs::CAP_ANY as isize,
    CapVFW = crate::ffi::cv::VideoCaptureAPIs::CAP_VFW as isize,
}

// impl VideoCaptureAPIs {
//     pub const CAP_V4L: VideoCaptureAPIs = VideoCaptureAPIs::CapVFW;
//     pub const CAP_V4L2: VideoCaptureAPIs = VideoCaptureAPIs::CapVFW;
// }

pub struct VideoCapture {
    inner: cxx::UniquePtr<crate::ffi::cv::VideoCapture>,
}

impl VideoCapture {
    pub fn new(index: i32, api_preference: VideoCaptureAPIs) -> Self {
        Self {
            inner: crate::ffi::cv::VideoCapture::new3(c_int(index), c_int(api_preference as i32)).within_unique_ptr(),
        }
    }
    pub fn stream_extraction(&mut self, mat: &mut Mat) {
        crate::ffi::wrapper::stream_extraction(self.inner.pin_mut(), mat.inner.pin_mut());
    }

    pub fn get_backend_name(&self) -> String {
        self.inner.getBackendName().to_string()
    }
}
