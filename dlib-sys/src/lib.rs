#![allow(unused_imports)]

pub mod cv_image;
pub mod frontal_face_detector;
pub mod image_window;
pub mod matrix;

autocxx::include_cpp! {
    #include "wrapper/cv_image.cpp"
    #include "wrapper/frontal_face_detector.cpp"
    #include "wrapper/image_window.cpp"
    #include "wrapper/matrix.cpp"
    #include "wrapper/rectangle.cpp"

    generate!("wrapper::CvImage")
    generate!("wrapper::FrontalFaceDetector")
    generate!("wrapper::ImageWindow")
    generate!("wrapper::Matrix")
    generate!("wrapper::Rectangle")

    // #include "dlib/geometry/rectangle.h"
    // #include "dlib/dnn.h"

    // generate!("dlib::rectangle")

    // generate!("wrapper::Matrix")

    // generate!("dlib::DLIB_USE_CUDA")



    extern_cpp_type!("cv::Mat", opencv_sys::ffi_extern::Mat)

    // #include "dlib/image_processing/frontal_face_detector.h"
    // #include "dlib/image_processing/object_detector.h"

    // generate!("dlib::frontal_face_detector")
    // generate!("dlib::get_frontal_face_detector")

    // block!("dlib::object_detector")

    safety!(unsafe)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        // ffi::dlib::image_window
    }
}
