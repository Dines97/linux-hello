use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{cv_image::CvImage, matrix::Matrix};

pub struct ShapePredictor {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::ShapePredictor>,
}

impl ShapePredictor {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::ShapePredictor::new().within_unique_ptr(),
        }
    }

    /// Perform operator() call on c++ side hence this terrible name
    /// NOTE: Find a better name for this method
    pub fn function_call(
        &mut self,
        cv_image: &mut CvImage,
        rectangles: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>>,
    ) -> UniquePtr<CxxVector<crate::ffi::wrapper::FullObjectDetection>> {
        self.inner
            .pin_mut()
            .functionCall(cv_image.inner.pin_mut(), rectangles)
    }
}
