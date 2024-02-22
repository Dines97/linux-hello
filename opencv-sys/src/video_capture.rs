use autocxx::prelude::*;

use crate::mat::Mat;

pub(super) enum VideoCaptureAPIs {
    CapAny = crate::ffi::cv::VideoCaptureAPIs::CAP_ANY as isize,
}

pub(super) struct VideoCapture {
    inner: cxx::UniquePtr<crate::ffi::cv::VideoCapture>,
}

impl VideoCapture {
    pub(super) fn new(index: i32, api_preference: VideoCaptureAPIs) -> Self {
        Self {
            inner: crate::ffi::cv::VideoCapture::new3(c_int(index), c_int(api_preference as i32))
                .within_unique_ptr(),
        }
    }
}

pub(super) fn stream_extraction(video_capture: &mut VideoCapture, mat: &mut Mat) {
    crate::ffi::wrapper::streamExtraction(video_capture.inner.pin_mut(), mat.inner.pin_mut());
}
