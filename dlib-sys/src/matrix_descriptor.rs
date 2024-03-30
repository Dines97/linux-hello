use autocxx::prelude::*;
use cxx::CxxVector;
use opencv_sys::mat::Mat;

pub struct MatrixDescriptor {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::MatrixDescriptor>,
}

impl MatrixDescriptor {
    pub fn to_vec(&self) -> Vec<f32> {
        let cxx_vector: UniquePtr<CxxVector<f32>> = self.inner.as_ref().unwrap().to_vector();
        cxx_vector.as_ref().unwrap().iter().copied().collect()
    }

    pub fn subtract(&self, matrix: &MatrixDescriptor) -> MatrixDescriptor {
        MatrixDescriptor {
            inner: self
                .inner
                .as_ref()
                .unwrap()
                .subtract(matrix.inner.as_ref().unwrap())
                .within_unique_ptr(),
        }
    }
    pub fn length(&self) -> f32 {
        return self.inner.as_ref().unwrap().length();
    }
}
impl core::fmt::Debug for MatrixDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MatrixDescriptor")
            .field("inner", &self.inner.debug())
            .finish()
    }
}
