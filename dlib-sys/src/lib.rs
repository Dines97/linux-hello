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
