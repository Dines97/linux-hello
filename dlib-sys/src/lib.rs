// dlib::cv_image<dlib::bgr_pixel> CVImage(cvIplImage(opencv_image));
// assign_image(dlib_image, CVImage);

// dlib::array2d<bgr_pixel> dlibFrame;
// dlib::assign_image(dlibFrame, dlib::cv_image<bgr_pixel>(temp));

pub mod frontal_face_detector;
pub mod image_window;
pub mod matrix;
pub mod rectangle;

autocxx::include_cpp! {
    #include "wrapper.hpp"

    generate!("wrapper::FrontalFaceDetector")
    generate!("wrapper::Matrix")
    generate!("wrapper::ImageWindow")
    generate!("wrapper::Rectangle")



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
