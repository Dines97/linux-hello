#![allow(unused_imports)]

pub mod gui;
pub mod mat;
pub mod video_capture;

autocxx::include_cpp! {
    #include "wrapper.hpp"
    #include "opencv2/highgui.hpp"
    #include "opencv2/videoio.hpp"
    #include "opencv2/core/mat.hpp"

    generate!("cv::namedWindow")
    generate!("cv::WindowFlags")
    generate!("wrapper::imshow")
    generate!("cv::waitKey")

    generate!("cv::VideoCapture")
    generate!("cv::VideoCaptureAPIs")

    generate!("cv::Mat")
    generate!("wrapper::streamExtraction")

    block!("cv::OutputArray")
    block!("cv::InputArray")

    safety!(unsafe)
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
        video_capture::{stream_extraction, VideoCapture, VideoCaptureAPIs},
    };

    #[test]
    fn it_works() {
        let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);

        let mut mat: Mat = Mat::new();

        named_window("hello");

        loop {
            stream_extraction(&mut video_capture, &mut mat);
            imshow("hello", &mut mat);
            wait_key(50);
        }
    }
}
