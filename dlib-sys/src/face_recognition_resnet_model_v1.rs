use std::fmt::Debug;

use autocxx::prelude::*;
use cxx::let_cxx_string;

use crate::{matrix::Matrix, matrix_descriptor::MatrixDescriptor};

pub struct FaceRecognitionResnetModelV1 {
    inner: cxx::UniquePtr<crate::ffi::wrapper::FaceRecognitionResnetModelV1>,
}

impl FaceRecognitionResnetModelV1 {
    pub fn new(file_path: String) -> Self {
        Self {
            inner: crate::ffi::wrapper::FaceRecognitionResnetModelV1::new(file_path).within_unique_ptr(),
        }
    }

    // TODO: Add vector support
    pub fn function_call(&self, matrix: &Matrix) -> MatrixDescriptor {
        MatrixDescriptor {
            inner: self
                .inner
                .function_call(matrix.inner.as_ref().unwrap())
                .within_unique_ptr(),
        }
    }
}
