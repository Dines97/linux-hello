use autocxx::as_copy;
use cxx::{SharedPtr, UniquePtr};

#[derive(Clone)]
pub struct Rectangle {
    pub(crate) inner: SharedPtr<crate::ffi::wrapper::Rectangle>,
}
