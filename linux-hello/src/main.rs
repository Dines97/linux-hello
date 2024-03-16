mod cycle_controller;

use cycle_controller::CycleController;
use dlib_sys::{
    cv_image::CvImage, frontal_face_detector::FrontalFaceDetector, image_window::ImageWindow,
};
use env_logger::{Builder, Target};
use opencv_sys::{
    mat::Mat,
    video_capture::{VideoCapture, VideoCaptureAPIs},
};

fn main() {
    Builder::from_default_env()
        .target(Target::Stdout)
        .filter_level(log::LevelFilter::Trace)
        .init();

    opencv_sys::set_num_threads(1);

    let mut video_capture: VideoCapture = VideoCapture::new(0, VideoCaptureAPIs::CapAny);

    log::info!("OpenCV backend name: {}", video_capture.get_backend_name());

    let mut mat: Mat = Mat::new();

    let mut image_window: ImageWindow = ImageWindow::new();

    let mut frontal_face_detector: FrontalFaceDetector = FrontalFaceDetector::new();

    let mut cycle_controller: CycleController = CycleController::new();

    loop {
        video_capture.stream_extraction(&mut mat);

        let mut cv_image: CvImage = CvImage::new(&mut mat);

        // std::thread::sleep(time::Duration::from_secs_f64(0.2));
        let rectangles = frontal_face_detector.function_call(&mut cv_image);

        image_window.clear_overlay();
        image_window.set_image(&mut cv_image);
        image_window.add_overlays(rectangles);

        cycle_controller.throttle(10.0);
        log::trace!("{}", cycle_controller);
        cycle_controller.update();
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
