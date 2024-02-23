use autocxx::prelude::*;
use cxx::let_cxx_string;

use crate::mat::Mat;

pub enum WindowFlags {
    WindowAutosize = crate::ffi::cv::WindowFlags::WINDOW_AUTOSIZE as isize,
}

pub fn named_window(winname: &str) {
    let_cxx_string!(_winname = winname);
    crate::ffi::cv::namedWindow(&_winname, c_int(WindowFlags::WindowAutosize as i32));
}

pub fn imshow(winname: &str, mat: &mut Mat) {
    let_cxx_string!(_winname = winname);
    crate::ffi::wrapper::imshow(&_winname, mat.inner.pin_mut());
}

pub fn wait_key(delay: i32) {
    crate::ffi::cv::waitKey(c_int(delay));
}
