use cxx::{CxxVector, UniquePtr};

pub struct FullObjectDetection {
    pub(crate) inner: UniquePtr<crate::ffi::wrapper::FullObjectDetection>,
}

impl FullObjectDetection {
    pub fn render_face_detections(&self) -> UniquePtr<CxxVector<crate::ffi::wrapper::OverlayLine>> {
        crate::ffi::wrapper::render_face_detections(self.inner.as_ref().unwrap())
    }
}
