use crate::core::display::Display;
use color_eyre::{eyre::Ok, Result};
use cycle_controller::CycleController;
use dlib_sys::cv_image::CvImage;

pub(crate) struct FaceRecognition {
    face_recognition: dlib_support::face_recognition::FaceRecognition,
    display: Option<Display>,
    cycle_controller: CycleController,
}

unsafe impl Send for FaceRecognition {}

impl FaceRecognition {
    pub(crate) fn new(enable_display: bool) -> Result<Self> {
        let config = crate::config::read();

        log::info!(
            "Shape predictor filepath: {}",
            config.models.shape_predictor.filepath.display()
        );
        log::info!(
            "Face recognition filepath: {}",
            config.models.face_recognition.filepath.display()
        );

        Ok(Self {
            face_recognition: dlib_support::face_recognition::FaceRecognition::new(
                &config.models.shape_predictor.filepath,
                &config.models.face_recognition.filepath,
            ),

            display: enable_display.then(Display::default),
            cycle_controller: CycleController::default(),
        })
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
