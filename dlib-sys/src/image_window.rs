use autocxx::prelude::*;
use cxx::CxxVector;

use crate::{
    cv_image::{self, CvImage},
    matrix::Matrix,
    overlay_line::OverlayLine,
    rectangle::Rectangle,
};

pub struct ImageWindow {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::ImageWindow>,
}

impl ImageWindow {
    pub fn new() -> Self {
        Self {
            inner: crate::ffi::wrapper::ImageWindow::new().within_unique_ptr(),
        }
    }

    pub fn set_image(&mut self, cv_image: &CvImage) {
        self.inner.pin_mut().set_image(cv_image.inner.as_ref().unwrap())
    }

    pub fn add_rectangle_overlay(&mut self, rectangle: Rectangle) {
        self.inner.pin_mut().add_rectangle_overlay(rectangle.inner);
    }

    pub fn add_line_overlay(
        &mut self,
        overlay_lines: UniquePtr<CxxVector<crate::ffi::wrapper::OverlayLine>>,
    ) {
        self.inner.pin_mut().add_line_overlay(overlay_lines);
    }

    pub fn clear_overlay(&mut self) {
        self.inner.pin_mut().clear_overlay();
    }
}
