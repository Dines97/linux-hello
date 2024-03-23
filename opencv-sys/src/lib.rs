#![allow(unused_imports)]

use autocxx::prelude::*;

pub mod gui;
pub mod mat;
pub mod video_capture;

autocxx::include_cpp! {
    #include "wrapper/core.cpp"
    #include "opencv2/highgui.hpp"
    #include "opencv2/videoio.hpp"
    #include "opencv2/core/mat.hpp"

    generate!("cv::namedWindow")
    generate!("cv::WindowFlags")
    generate!("wrapper::imshow")
    generate!("cv::waitKey")
    generate!("cv::setNumThreads")

    generate!("cv::VideoCapture")
    generate!("cv::VideoCaptureAPIs")

    generate!("cv::Mat")
    generate!("wrapper::stream_extraction")

    block!("cv::OutputArray")
    block!("cv::InputArray")

    safety!(unsafe)
}

pub fn set_num_threads(nthreads: i32) {
    ffi::cv::setNumThreads(c_int(nthreads));
}

pub mod ffi_extern {

    use crate::*;

    // Required for dlib
    pub use ffi::cv::Mat;
}

#[cfg(test)]
mod tests {

    use crate::*;

    use self::{
        gui::{imshow, named_window, wait_key},
        mat::Mat,
        video_capture::{VideoCapture, VideoCaptureAPIs},
    };

    // #[test]
    // fn it_works() {
    //     let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);
    //
    //     let mut mat: Mat = Mat::new();
    //
    //     named_window("hello");
    //
    //     video_capture.stream_extraction(&mut mat);
    //     imshow("hello", &mut mat);
    //     wait_key(50);
    // }
}
