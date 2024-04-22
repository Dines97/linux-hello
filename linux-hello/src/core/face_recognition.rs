use dlib_support::face_recognition::FaceRecognition;
use dlib_sys::image_window::ImageWindow;
use railwork::transform::Transform;

use crate::{config::GLOBAL_CONFIG, cycle_controller::CycleController};

pub(crate) struct FaceRecogntion {
    face_recognition: FaceRecognition,

    image_window: ImageWindow,
    face_image_window: ImageWindow,
    prev: Vec<f32>,
    cycle_controller: CycleController,
}

unsafe impl Send for FaceRecogntion {}

impl Default for FaceRecogntion {
    fn default() -> Self {
        let config = GLOBAL_CONFIG.read().unwrap();

        log::info!("Models dir: {}", config.models.models_dir.display());
        log::info!(
            "Shape predictor file path: {}",
            config.models.shape_predictor.file_name.display()
        );
        log::info!(
            "Face recognition file path: {}",
            config.models.face_recognition.file_name.display()
        );

        Self {
            face_recognition: FaceRecognition::new(
                config.models.models_dir.join(&config.models.shape_predictor.file_name),
                config.models.models_dir.join(&config.models.face_recognition.file_name),
            ),

            image_window: ImageWindow::new(),
            face_image_window: ImageWindow::new(),
            prev: vec![0_f32; 128],
            cycle_controller: CycleController::new(),
        }
    }
}

impl Transform for FaceRecogntion {
    type Input = dlib_sys::cv_image::CvImage;
    type Output = std::vec::Vec<dlib_support::face::Face>;

    fn run(&mut self, input: Self::Input) -> Self::Output {
        let faces = self.face_recognition.get_faces(&input);

        self.image_window.clear_overlay();
        self.image_window.set_cv_image(&input);

        log::trace!("Face recognition {}", self.cycle_controller);
        self.cycle_controller.update();

        faces.iter().for_each(|x| {
            self.image_window.add_rectangle_overlay(x.rectangle.clone());
            let overlays = x.full_object_detection.render_face_detections();
            self.image_window.add_line_overlay(overlays);
            self.face_image_window.set_matrix(&x.face_chip);

            log::trace!(
                "Accuracy: {}",
                euclidean_distance(&x.face_descriptor.to_vec(), &self.prev.to_vec())
            );

            self.prev = x.face_descriptor.to_vec();
        });

        faces
    }
}

fn euclidean_distance(left: &[f32], right: &[f32]) -> f32 {
    assert!(left.len() == right.len(), "Vectors must have the same length");

    let sum_of_squares: f32 = std::iter::zip(left, right).map(|(&x, &y)| (x - y).powi(2)).sum();

    sum_of_squares.sqrt()
}
