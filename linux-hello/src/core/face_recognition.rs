use crate::{config::GLOBAL_CONFIG, core::display::Display, cycle_controller::CycleController};
use railwork::transform::Transform;

pub(crate) struct FaceRecognition {
    face_recognition: dlib_support::face_recognition::FaceRecognition,
    display: Option<Display>,
    cycle_controller: CycleController,
    prev: Vec<f32>,
}

unsafe impl Send for FaceRecognition {}

impl FaceRecognition {
    pub(crate) fn new(enable_display: bool) -> Self {
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
            face_recognition: dlib_support::face_recognition::FaceRecognition::new(
                config.models.models_dir.join(&config.models.shape_predictor.file_name),
                config.models.models_dir.join(&config.models.face_recognition.file_name),
            ),

            display: enable_display.then(Display::default),
            cycle_controller: CycleController::new(),
            prev: vec![0_f32; 128],
        }
    }
}

impl Transform for FaceRecognition {
    type Input = dlib_sys::cv_image::CvImage;
    type Output = Vec<dlib_support::face::Face>;

    fn run(&mut self, input: Self::Input) -> Self::Output {
        let faces = self.face_recognition.get_faces(&input);

        if let Some(ref mut x) = self.display {
            x.clear();
            x.display(&input);
        }

        log::trace!("Face recognition {}", &self.cycle_controller);
        self.cycle_controller.update();

        faces.iter().for_each(|face| {
            let overlays = face.full_object_detection.render_face_detections();

            if let Some(ref mut x) = self.display {
                x.overlay(face.rectangle.clone(), overlays);
                x.display_face(&face.face_chip);
            }

            log::trace!(
                "Accuracy: {}",
                euclidean_distance(&face.face_descriptor.to_vec(), &self.prev.to_vec())
            );

            self.prev = face.face_descriptor.to_vec();
        });

        faces
    }
}

fn euclidean_distance(left: &[f32], right: &[f32]) -> f32 {
    assert!(left.len() == right.len(), "Vectors must have the same length");

    let sum_of_squares: f32 = std::iter::zip(left, right).map(|(&x, &y)| (x - y).powi(2)).sum();

    sum_of_squares.sqrt()
}
