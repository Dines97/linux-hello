use crate::{config::GLOBAL_CONFIG, core::display::Display};
use cycle_controller::CycleController;
use dlib_sys::cv_image::CvImage;

pub(crate) struct FaceRecognition {
    face_recognition: dlib_support::face_recognition::FaceRecognition,
    display: Option<Display>,
    cycle_controller: CycleController,
}

unsafe impl Send for FaceRecognition {}

impl FaceRecognition {
    pub(crate) fn new(enable_display: bool) -> Self {
        let config = GLOBAL_CONFIG.read().unwrap();

        log::info!("Models dir: {}", config.models.dir.display());
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
                config.models.dir.join(&config.models.shape_predictor.file_name),
                config.models.dir.join(&config.models.face_recognition.file_name),
            ),

            display: enable_display.then(Display::default),
            cycle_controller: CycleController::default(),
        }
    }

    pub(crate) fn run(&mut self, input: CvImage) -> Vec<dlib_support::face::Face> {
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
        });

        faces
    }
}
