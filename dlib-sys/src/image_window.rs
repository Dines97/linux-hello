use autocxx::prelude::*;

use crate::{matrix::Matrix, rectangle::Rectangles};

pub struct ImageWindow {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::ImageWindow>,
}

impl ImageWindow {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::ImageWindow::new().within_unique_ptr(),
        }
    }

    pub fn set_image(&mut self, matrix: &mut Matrix) {
        self.inner.pin_mut().setImage(matrix.inner.pin_mut())
    }

    pub fn add_overlays(&mut self, rectangles: &mut Rectangles) {
        self.inner.pin_mut().addOverlay1(rectangles.inner.pin_mut());
    }

    pub fn clear_overlay(&mut self) {
        self.inner.pin_mut().clearOverlay();
    }
}
