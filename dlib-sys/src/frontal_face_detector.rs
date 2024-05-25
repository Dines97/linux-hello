use std::pin::Pin;

use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{cv_image::CvImage, matrix::Matrix, rectangle::Rectangle};

pub struct FrontalFaceDetector {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::FrontalFaceDetector>,
}

impl FrontalFaceDetector {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::FrontalFaceDetector::new().within_unique_ptr(),
        }
    }

    /// Perform operator() call on c++ side hence this terrible name
    /// NOTE: Find a better name for this method
    pub fn function_call(&mut self, cv_image: &CvImage) -> Vec<Rectangle> {
        // let mut cxx_rectangles: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> =
        //     self.inner.function_call(cv_image.inner.pin_mut());

        let rectangles: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> =
            self.inner.pin_mut(). function_call(cv_image.inner.as_ref().unwrap());

        return rectangles
            .as_ref()
            .unwrap()
            .iter()
            .map(|x: &crate::ffi::wrapper::Rectangle| Rectangle { inner: x.clone() })
            .collect::<Vec<_>>();
    }
}

impl Default for FrontalFaceDetector {
    fn default() -> Self {
        Self::new()
    }
}
