use crate::{ffi, matrix::Matrix, matrix_descriptor::MatrixDescriptor};
use autocxx::prelude::*;
use cxx::let_cxx_string;
use std::{fmt::Debug, pin::Pin};

pub struct FaceRecognitionResnetModelV1 {
    inner: Pin<Box<crate::ffi::wrapper::FaceRecognitionResnetModelV1>>,
}

impl FaceRecognitionResnetModelV1 {
    pub fn new<T: ffi::ToCppString>(file_path: T) -> Self {
        Self {
            inner: crate::ffi::wrapper::FaceRecognitionResnetModelV1::new(file_path).within_box(),
        }
    }

    // TODO: Add vector support
    pub fn function_call(&mut self, matrix: &Matrix) -> MatrixDescriptor {
        MatrixDescriptor {
            inner: self.inner.as_mut().function_call(&matrix.inner.as_ref()).within_box(),
        }
    }
}

unsafe impl Send for FaceRecognitionResnetModelV1 {}
