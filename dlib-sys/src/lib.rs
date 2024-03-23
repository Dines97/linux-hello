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

    generate!("wrapper::FrontalFaceDetector")
    generate_pod!("wrapper::Rectangle")

    generate!("wrapper::ShapePredictor")
    generate!("wrapper::FullObjectDetection")

    generate!("wrapper::render_face_detections")
    generate!("wrapper::OverlayLine")

    generate!("wrapper::CvImage")
    generate!("wrapper::ImageWindow")
    generate!("wrapper::Matrix")

    extern_cpp_type!("cv::Mat", opencv_sys::ffi_extern::Mat)

    safety!(unsafe)
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        // ffi::dlib::image_window
    }
}
