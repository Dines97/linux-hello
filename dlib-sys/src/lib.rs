#![allow(unused_imports)]

use cxx::{CxxVector, UniquePtr};
use full_object_detection::FullObjectDetection;
use overlay_line::OverlayLine;

pub mod cv_image;
pub mod frontal_face_detector;
pub mod full_object_detection;
pub mod image_window;
pub mod matrix;
pub mod overlay_line;
pub mod rectangle;
pub mod shape_predictor;

autocxx::include_cpp! {
    #include "wrapper/core.cpp"
    #include "wrapper/cv_image.cpp"
    #include "wrapper/frontal_face_detector.cpp"
    #include "wrapper/image_window.cpp"
    #include "wrapper/matrix.cpp"
    #include "wrapper/overlay_line.cpp"
    #include "wrapper/rectangle.cpp"
    #include "wrapper/shape_predictor.cpp"

    generate!("wrapper::render_face_detections")
    generate!("wrapper::CvImage")
    generate!("wrapper::FrontalFaceDetector")
    generate!("wrapper::ImageWindow")
    generate!("wrapper::Matrix")
    generate!("wrapper::OverlayLine")
    generate!("wrapper::Rectangle")
    generate!("wrapper::RectanglesWrapper")
    generate!("wrapper::ShapePredictor")

    extern_cpp_type!("cv::Mat", opencv_sys::ffi_extern::Mat)

    safety!(unsafe)
}

pub fn render_face_detections(
    full_object_detection: &mut FullObjectDetection,
) -> UniquePtr<CxxVector<crate::ffi::wrapper::OverlayLine>> {
    crate::ffi::wrapper::render_face_detections(full_object_detection.inner.pin_mut())
    // OverlayLine {
    //     inner: crate::ffi::wrapper::render_face_detections(full_object_detection.inner.pin_mut()),
    // }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        // ffi::dlib::image_window
    }
}
