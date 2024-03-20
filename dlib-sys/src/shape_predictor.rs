use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{
    cv_image::CvImage, full_object_detection::FullObjectDetection, matrix::Matrix,
    rectangle::Rectangle,
};

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
        &self,
        cv_image: &mut CvImage,
        rectangle: Rectangle,
    ) -> FullObjectDetection {
        FullObjectDetection {
            inner: self
                .inner
                .function_call(cv_image.inner.pin_mut(), rectangle.inner)
                .within_unique_ptr(),
        }
    }
}
