mod cycle_controller;

use cycle_controller::CycleController;
use dlib::face_recognition::FaceRecognition;
use dlib_sys::{cv_image::CvImage, image_window::ImageWindow};
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
    let mut face_image_window: ImageWindow = ImageWindow::new();

    let mut cycle_controller: CycleController = CycleController::new();

    let face_recognition: FaceRecognition = FaceRecognition::new();

    loop {
        video_capture.stream_extraction(&mut mat);

        let cv_image: CvImage = CvImage::new(&mat);

        let faces = face_recognition.get_faces(&cv_image);

        image_window.clear_overlay();
        faces.iter().for_each(|x| {
            image_window.add_rectangle_overlay(x.rectangle.clone());
            let overlays = x.full_object_detection.render_face_detections();
            image_window.add_line_overlay(overlays);
            face_image_window.set_matrix(&x.face_chip);
        });
        image_window.set_cv_image(&cv_image);

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
