use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxVector};

use crate::{cv_image::CvImage, full_object_detection::FullObjectDetection, matrix::Matrix, rectangle::Rectangle};

pub struct ShapePredictor {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::ShapePredictor>,
}

impl ShapePredictor {
    pub fn new(file_path: String) -> Self {
        Self {
            inner: crate::ffi::wrapper::ShapePredictor::new(file_path).within_unique_ptr(),
        }
    }

    /// Perform operator() call on c++ side hence this terrible name
    /// NOTE: Find a better name for this method
    pub fn function_call(&self, cv_image: &CvImage, rectangle: Rectangle) -> FullObjectDetection {
        FullObjectDetection {
            inner: self
                .inner
                .function_call(cv_image.inner.as_ref().unwrap(), rectangle.inner)
                .within_unique_ptr(),
        }
    }
}
