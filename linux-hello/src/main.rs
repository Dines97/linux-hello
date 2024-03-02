use core::fmt;
use std::time::{Duration, Instant};

use dlib_sys::{
    cv_image::CvImage, frontal_face_detector::FrontalFaceDetector, image_window::ImageWindow,
    rectangle::Rectangles,
};
use opencv_sys::{
    mat::Mat,
    video_capture::{stream_extraction, VideoCapture, VideoCaptureAPIs},
};

struct CycleController {
    cps: i32,
    instant: Instant,
    // previous: Instant
    duration: Duration,
}

impl CycleController {
    pub fn new(cps: i32) -> Self {
        Self {
            cps,
            instant: Instant::now(),
            duration: Duration::ZERO, // previous: Instant::now()
        }
    }

    pub fn update(&mut self) {
        self.duration = self.instant.elapsed();

        self.instant = Instant::now();
    }
}

impl fmt::Display for CycleController {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cps: f64 = 1_f64 / f64::max(self.duration.as_secs_f64(), f64::MIN_POSITIVE);

        write!(f, "CPS: {}", cps)
    }
}

fn main() -> ! {
    let mut video_capture: VideoCapture = VideoCapture::new(2, VideoCaptureAPIs::CapAny);

    let mut mat: Mat = Mat::new();

    let mut image_window: ImageWindow = ImageWindow::new();

    let mut frontal_face_detector: FrontalFaceDetector = FrontalFaceDetector::new();

    let mut cycle_controller: CycleController = CycleController::new(30);

    loop {
        stream_extraction(&mut video_capture, &mut mat);

        cycle_controller.update();
        println!("{}", cycle_controller);

        let mut cv_image: CvImage = CvImage::new(&mut mat);

        // std::thread::sleep(time::Duration::from_secs_f64(0.2));
        let mut rectangles: Rectangles = frontal_face_detector.function_call(&mut cv_image);

        image_window.clear_overlay();
        image_window.set_image(&mut cv_image);
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
