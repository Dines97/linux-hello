use cxx::UniquePtr;

pub struct FullObjectDetection {
    pub(crate) inner: UniquePtr<crate::ffi::wrapper::FullObjectDetection>,
}
