use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{cv_image::CvImage, matrix::Matrix};

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
    pub fn function_call(
        &mut self,
        cv_image: &mut CvImage,
    ) -> UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> {
        self.inner.pin_mut().functionCall(cv_image.inner.pin_mut())

        // let cxx_vector: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> =
        //     self.inner.pin_mut().functionCall(cv_image.inner.pin_mut());

        // cxx_vector
        //     .pin_mut()
        //     .iter()
        //     .map(|&x| Rectangle { inner: x })
        //     .collect()

        // let rust_vector: Vec<UniquePtr<crate::ffi::wrapper::Rectangle>> = cxx_vector
        //     .pin_mut()
        //     .iter()
        //     .map(|x: crate::ffi::wrapper::Rectangle| UniquePtr::emplace(x))
        //     .collect();

        // let rust_vector2: Vec<Pin<Box<crate::ffi::wrapper::Rectangle>>> = cxx_vector
        //     .pin_mut()
        //     .iter()
        //     .map(|x: crate::ffi::wrapper::Rectangle| Box::emplace(x))
        //     .collect();

        // for x in cxx_vector {}

        // let mut obj = crate::ffi::wrapper::Rectangle::new().within_unique_ptr();

        // let rust_vector = cxx_vector
        //     .pin_mut()
        //     .iter()
        //     .map(|x| UniquePtr::emplace(x))
        //     .collect();

        // rust_vector

        // let rectangle: Rectangle = Rectangle {inner: UniquePtr::emplace(a.p) };
        //
        //
        // Rectangle {inner: a[0] };
        //
        // let vec_cxx_rectangles: UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>> =
        //     self.inner.pin_mut().functionCall(cv_image.inner.pin_mut());
        //
        // UniquePtr::new(value)
    }
}
