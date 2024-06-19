use autocxx::prelude::*;
use opencv_sys::mat::Mat;
use std::pin::Pin;

pub struct Matrix {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::Matrix>>,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::Matrix::new().within_box(),
        }
    }
}

unsafe impl Send for Matrix {}
