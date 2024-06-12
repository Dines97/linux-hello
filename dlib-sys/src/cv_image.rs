use autocxx::prelude::*;
use opencv_sys::mat::Mat;
use std::pin::Pin;

pub struct CvImage {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::CvImage>>,

    // Keep original opencv image or it will go out of scope.
    // FIX: Maybe fix this somehow on c++ bridge side?
    _original: opencv_sys::mat::Mat,
}

impl From<opencv_sys::mat::Mat> for CvImage {
    fn from(value: opencv_sys::mat::Mat) -> Self {
        Self {
            inner: crate::ffi::wrapper::CvImage::new(&value.inner.as_ref()).within_box(),
            _original: value,
        }
    }
}

unsafe impl Send for CvImage {}
