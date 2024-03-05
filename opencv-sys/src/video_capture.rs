use autocxx::prelude::*;

use crate::mat::Mat;

pub enum VideoCaptureAPIs {
    CapAny = crate::ffi::cv::VideoCaptureAPIs::CAP_ANY as isize,
}

pub struct VideoCapture {
    inner: cxx::UniquePtr<crate::ffi::cv::VideoCapture>,
}

impl VideoCapture {
    pub fn new(index: i32, api_preference: VideoCaptureAPIs) -> Self {
        Self {
            inner: crate::ffi::cv::VideoCapture::new3(c_int(index), c_int(api_preference as i32))
                .within_unique_ptr(),
        }
    }
    pub fn stream_extraction(&mut self, mat: &mut Mat) {
        crate::ffi::wrapper::streamExtraction(self.inner.pin_mut(), mat.inner.pin_mut());
    }

    pub fn get_backend_name(&self) -> String {
        self.inner.getBackendName().to_string()
    }
}
