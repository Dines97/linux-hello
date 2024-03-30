#![allow(unused_imports)]

use autocxx::c_ulong;
use chip_details::ChipDetails;
use cv_image::CvImage;
use cxx::{CxxVector, UniquePtr};
use full_object_detection::FullObjectDetection;
use matrix::Matrix;
use overlay_line::OverlayLine;

pub mod chip_details;
pub mod cv_image;
pub mod face_recognition_resnet_model_v1;
pub mod frontal_face_detector;
pub mod full_object_detection;
pub mod image_window;
pub mod matrix;
pub mod matrix_descriptor;
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
    #include "wrapper/face_recognition_resnet_model_v1.cpp"

    generate!("wrapper::FrontalFaceDetector")
    generate_pod!("wrapper::Rectangle")

    generate!("wrapper::ShapePredictor")
    generate!("wrapper::FullObjectDetection")

    generate!("wrapper::render_face_detections")
    generate!("wrapper::OverlayLine")

    generate!("wrapper::FaceRecognitionResnetModelV1")

    generate!("wrapper::CvImage")
    generate!("wrapper::ImageWindow")
    generate!("wrapper::Matrix")
    generate!("wrapper::MatrixDescriptor")

    generate_pod!("wrapper::ChipDetails")

    generate!("wrapper::extract_image_chip")
    generate!("wrapper::get_face_chip_details")

    extern_cpp_type!("cv::Mat", opencv_sys::ffi_extern::Mat)

    safety!(unsafe)
}

pub fn get_face_chip_details(
    full_object_detection: &full_object_detection::FullObjectDetection,
    size: u64,
    padding: f64,
) -> ChipDetails {
    ChipDetails {
        inner: crate::ffi::wrapper::get_face_chip_details(
            full_object_detection.inner.as_ref().unwrap(),
            c_ulong(size),
            padding,
        ),
    }
}

pub fn extract_image_chip(cv_image: &CvImage, chip_details: &ChipDetails, image_chip: &mut Matrix) {
    crate::ffi::wrapper::extract_image_chip(
        cv_image.inner.as_ref().unwrap(),
        &chip_details.inner,
        image_chip.inner.pin_mut(),
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        // ffi::dlib::image_window
    }
}
