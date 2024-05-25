use cxx::{CxxVector, UniquePtr};
use dlib_sys::{cv_image, image_window::ImageWindow, matrix, rectangle::Rectangle};

pub(crate) struct Display {
    image_window: ImageWindow,
    face_image_window: ImageWindow,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            image_window: Default::default(),
            face_image_window: Default::default(),
        }
    }
}

impl Display {
    pub(crate) fn display(&mut self, cv_image: &cv_image::CvImage) {
        self.image_window.set_cv_image(cv_image);
    }

    pub(crate) fn display_face(&mut self, matrix: &matrix::Matrix) {
        self.face_image_window.set_matrix(matrix);
    }

    pub(crate) fn overlay(
        &mut self,
        rectangle: Rectangle,
        overlay_lines: UniquePtr<CxxVector<dlib_sys::ffi_extern::OverlayLine>>,
    ) {
        self.image_window.add_rectangle_overlay(rectangle);
        self.image_window.add_line_overlay(overlay_lines);
    }

    pub(crate) fn clear(&mut self) {
        self.image_window.clear_overlay();
    }
}
