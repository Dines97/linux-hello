use cxx::UniquePtr;

pub struct OverlayLine {
    pub(crate) inner: UniquePtr<crate::ffi::wrapper::OverlayLine>,
}
