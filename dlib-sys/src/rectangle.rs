use autocxx::as_copy;
use cxx::{SharedPtr, UniquePtr};

pub struct Rectangle {
    pub(crate) inner: crate::ffi::wrapper::Rectangle,
}

impl Clone for Rectangle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
