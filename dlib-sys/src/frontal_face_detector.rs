use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{matrix::Matrix, rectangle::Rectangles};

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
    pub fn function_call(&mut self, matrix: &mut Matrix) -> Rectangles {
        let cxx_rectangles: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> =
            self.inner.pin_mut().functionCall(matrix.inner.pin_mut());

        Rectangles {
            inner: cxx_rectangles,
        }
    }
}
