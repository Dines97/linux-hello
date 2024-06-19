use crate::{cv_image::CvImage, ffi, full_object_detection::FullObjectDetection, rectangle::Rectangle};
use autocxx::prelude::*;
use cxx::{let_cxx_string, CxxVector};
use std::{path::Path, pin::Pin};

pub struct ShapePredictor {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::ShapePredictor>>,
}

impl ShapePredictor {
    pub fn new<T: ffi::ToCppString>(file_path: T) -> Self {
        Self {
            inner: crate::ffi::wrapper::ShapePredictor::new(file_path).within_box(),
        }
    }

    /// Perform operator() call on c++ side hence this terrible name
    /// NOTE: Find a better name for this method
    pub fn function_call(&mut self, cv_image: &CvImage, rectangle: Rectangle) -> FullObjectDetection {
        FullObjectDetection {
            inner: self
                .inner
                .as_mut()
                .function_call(&cv_image.inner.as_ref(), rectangle.inner)
                .within_unique_ptr(),
        }
    }
}

unsafe impl Send for ShapePredictor {}
