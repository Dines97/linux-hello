#![allow(unused_imports, dead_code)]

use std::pin::Pin;

use autocxx::prelude::*;
use cxx::let_cxx_string;

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

enum VideoCaptureAPIs {
    CapAny = ffi::cv::VideoCaptureAPIs::CAP_ANY as isize,
}

struct VideoCapture {
    inner: Pin<Box<ffi::cv::VideoCapture>>,
}

impl VideoCapture {
    fn new(index: i32, api_preference: VideoCaptureAPIs) -> Self {
        Self {
            inner: ffi::cv::VideoCapture::new3(c_int(index), c_int(api_preference as i32))
                .within_box(),
        }
    }
}

struct Mat {
    inner: Pin<Box<ffi::cv::Mat>>,
}

impl Mat {
    fn new() -> Self {
        Self {
            inner: ffi::cv::Mat::new().within_box(),
        }
    }
}

enum WindowFlags {
    WindowAutosize = ffi::cv::WindowFlags::WINDOW_AUTOSIZE as isize,
}

fn named_window(winname: &str) {
    let_cxx_string!(_winname = winname);
    ffi::cv::namedWindow(&_winname, c_int(WindowFlags::WindowAutosize as i32));
}

fn imshow(winname: &str, mat: &mut Mat) {
    let_cxx_string!(_winname = winname);
    ffi::wrapper::imshow(&_winname, mat.inner.as_mut());
}

fn stream_extraction(video_capture: &mut VideoCapture, mat: &mut Mat) {
    ffi::wrapper::streamExtraction(video_capture.inner.as_mut(), mat.inner.as_mut());
}

fn wait_key(delay: i32) {
    ffi::cv::waitKey(c_int(delay));
}

#[cfg(test)]
mod tests {

    use super::*;

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
