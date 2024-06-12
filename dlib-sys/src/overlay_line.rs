use std::pin::Pin;

pub struct OverlayLine {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::OverlayLine>>,
}
