use opencv_sys::{
    gui::{imshow, named_window, wait_key},
    mat::Mat,
    video_capture::{stream_extraction, VideoCapture, VideoCaptureAPIs},
};

fn main() {
    let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);

    let mut mat: Mat = Mat::new();

    named_window("hello");

    loop {
        stream_extraction(&mut video_capture, &mut mat);
        imshow("hello", &mut mat);
        wait_key(50);
    }
}
