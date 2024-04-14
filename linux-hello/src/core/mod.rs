use std::sync::mpsc::Receiver;

use color_eyre::eyre::Result;
use dlib_sys::{cv_image::CvImage, image_window::ImageWindow};

use crate::cycle_controller::CycleController;

use self::{face_recognition::FaceRecogntion, video_capture_thread::VideoCaptureThread};

mod face_recognition;
mod video_capture_thread;

pub(crate) struct Core {
    pub(crate) video_capture_thread: Option<VideoCaptureThread>,
    pub(crate) face_recognition: FaceRecogntion,
    pub(crate) receiver: Receiver<CvImage>,
}

impl Default for Core {
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        Self {
            video_capture_thread: Some(VideoCaptureThread::new(sender)),
            face_recognition: Default::default(),
            receiver,
        }
    }
}

impl Core {
    pub(crate) fn run(&mut self) -> Result<()> {
        let mut image_window: ImageWindow = ImageWindow::new();
        let mut face_image_window: ImageWindow = ImageWindow::new();

        let mut cycle_controller: CycleController = CycleController::new();

        let mut prev: Vec<f32> = vec![0_f32; 128];

        let mut counter = 0;

        'main: loop {
            counter += 1;
            if counter > 100 {
                drop(self.video_capture_thread.take());
            }

            let cv_image = self.receiver.recv().expect("Unable receive camera stream");

            let faces = self.face_recognition.execute(&cv_image);

            image_window.clear_overlay();

            faces.iter().for_each(|x| {
                image_window.add_rectangle_overlay(x.rectangle.clone());
                let overlays = x.full_object_detection.render_face_detections();
                image_window.add_line_overlay(overlays);
                face_image_window.set_matrix(&x.face_chip);

                euclidean_distance(&x.face_descriptor.to_vec(), &prev.to_vec());

                // log::trace!(
                //     "Accuracy: {}",
                //     euclidean_distance(&x.face_descriptor.to_vec(), &prev.to_vec())
                // );

                prev = x.face_descriptor.to_vec();
            });
            image_window.set_cv_image(&cv_image);

            // cycle_controller.throttle(10.0);
            log::trace!("Face recognition {}", cycle_controller);
            cycle_controller.update();

            if image_window.is_closed() {
                break 'main;
            }
        }

        Ok(())
    }
}

fn euclidean_distance(left: &[f32], right: &[f32]) -> f32 {
    assert!(left.len() == right.len(), "Vectors must have the same length");

    let sum_of_squares: f32 = std::iter::zip(left, right).map(|(&x, &y)| (x - y).powi(2)).sum();

    sum_of_squares.sqrt()
}
