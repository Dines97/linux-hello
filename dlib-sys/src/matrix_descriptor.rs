use autocxx::prelude::*;
use cxx::CxxVector;
use opencv_sys::mat::Mat;
use std::pin::Pin;

pub struct MatrixDescriptor {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::MatrixDescriptor>>,
}

impl MatrixDescriptor {
    pub fn to_vec(&self) -> Vec<f32> {
        let cxx_vector: UniquePtr<CxxVector<f32>> = self.inner.as_ref().to_vector();
        cxx_vector.as_ref().unwrap().iter().copied().collect()
    }

    pub fn subtract(&self, matrix: &MatrixDescriptor) -> MatrixDescriptor {
        MatrixDescriptor {
            inner: self.inner.as_ref().subtract(&matrix.inner.as_ref()).within_box(),
        }
    }
    pub fn length(&self) -> f32 {
        return self.inner.as_ref().length();
    }
}
impl core::fmt::Debug for MatrixDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatrixDescriptor")
            .field("inner", &self.inner.debug())
            .finish()
    }
}

unsafe impl Send for MatrixDescriptor {}
