use dlib_sys::{
    frontal_face_detector::FrontalFaceDetector, image_window::ImageWindow, matrix::Matrix,
    rectangle::Rectangles,
};
use opencv_sys::{
    mat::Mat,
    video_capture::{stream_extraction, VideoCapture, VideoCaptureAPIs},
};

fn main() {
    let mut video_capture: VideoCapture = VideoCapture::new(0, VideoCaptureAPIs::CapAny);

    let mut mat: Mat = Mat::new();

    let mut image_window: ImageWindow = ImageWindow::new();

    let mut frontal_face_detector: FrontalFaceDetector = FrontalFaceDetector::new();

    loop {
        stream_extraction(&mut video_capture, &mut mat);
        let mut matrix: Matrix = Matrix::from_opencv(&mut mat);

        let mut rectangles: Rectangles = frontal_face_detector.function_call(&mut matrix);

        image_window.clear_overlay();
        image_window.set_image(&mut matrix);
        image_window.add_overlays(&mut rectangles);
    }
}

// fn main() {
//     let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);
//
//     let mut mat: Mat = Mat::new();
//
//     named_window("hello");
//
//     loop {
//         stream_extraction(&mut video_capture, &mut mat);
//         imshow("hello", &mut mat);
//         wait_key(50);
//     }
// }
