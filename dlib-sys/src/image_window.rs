use crate::{
    cv_image::{self, CvImage},
    matrix::Matrix,
    overlay_line::OverlayLine,
    rectangle::Rectangle,
};
use autocxx::prelude::*;
use cxx::CxxVector;
use std::pin::Pin;

pub struct ImageWindow {
    pub(crate) inner: Pin<Box<crate::ffi::wrapper::ImageWindow>>,
}

impl Default for ImageWindow {
    fn default() -> Self {
        Self {
            inner: crate::ffi::wrapper::ImageWindow::new().within_box(),
        }
    }
}

impl ImageWindow {
    pub fn set_matrix(&mut self, matrix: &Matrix) {
        self.inner.as_mut().set_matrix(&matrix.inner.as_ref())
    }

    pub fn set_cv_image(&mut self, cv_image: &CvImage) {
        self.inner.as_mut().set_cv_image(&cv_image.inner.as_ref())
    }

    pub fn add_rectangle_overlay(&mut self, rectangle: Rectangle) {
        self.inner.as_mut().add_rectangle_overlay(rectangle.inner);
    }

    pub fn add_line_overlay(&mut self, overlay_lines: UniquePtr<CxxVector<crate::ffi::wrapper::OverlayLine>>) {
        self.inner.as_mut().add_line_overlay(overlay_lines);
    }

    pub fn clear_overlay(&mut self) {
        self.inner.as_mut().clear_overlay();
    }

    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

unsafe impl Send for ImageWindow {}
