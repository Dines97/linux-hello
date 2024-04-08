use crate::Runnable;
use crate::{config::GLOBAL_CONFIG, cycle_controller::CycleController};

use clap::Args;
use color_eyre::Result;
use dlib::face_recognition::FaceRecognition;
use dlib_sys::{cv_image::CvImage, image_window::ImageWindow};
use opencv_sys::{
    mat::Mat,
    video_capture::{VideoCapture, VideoCaptureAPIs},
};

#[derive(Debug, Args)]
pub(crate) struct TestArgs {
    #[arg(short, long)]
    pub camera: Option<i32>,
}

impl Runnable for TestArgs {
    fn run(&self) -> Result<()> {
        let config = GLOBAL_CONFIG.read().unwrap();

        let mut video_capture: VideoCapture = VideoCapture::new(
            self.camera.unwrap_or(config.video.camera_index),
            config.video.video_capture_api,
        );

        log::info!("OpenCV backend name: {}", video_capture.get_backend_name());

        let mut mat: Mat = Mat::new();

        let mut image_window: ImageWindow = ImageWindow::new();
        let mut face_image_window: ImageWindow = ImageWindow::new();

        let mut cycle_controller: CycleController = CycleController::new();

        let face_recognition: FaceRecognition = FaceRecognition::new(
            config.models.shape_predictor.file_path.clone(),
            config.models.face_recognition.file_path.clone(),
        );

        let mut prev: Vec<f32> = vec![0_f32; 128];

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

                log::trace!(
                    "Accuracy: {}",
                    euclidean_distance(&x.face_descriptor.to_vec(), &prev.to_vec())
                );

                prev = x.face_descriptor.to_vec();
            });
            image_window.set_cv_image(&cv_image);

            cycle_controller.throttle(10.0);
            // log::trace!("{}", cycle_controller);
            cycle_controller.update();
        }
    }
}

fn euclidean_distance(left: &[f32], right: &[f32]) -> f32 {
    assert!(left.len() == right.len(), "Vectors must have the same length");

    let sum_of_squares: f32 = std::iter::zip(left, right).map(|(&x, &y)| (x - y).powi(2)).sum();

    sum_of_squares.sqrt()
}
