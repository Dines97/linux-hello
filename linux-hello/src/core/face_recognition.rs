use dlib_support::face_recognition::FaceRecognition;
use dlib_sys::cv_image::CvImage;

use crate::config::GLOBAL_CONFIG;

pub(crate) struct FaceRecogntion {
    face_recognition: FaceRecognition,
}

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
        }
    }
}

impl FaceRecogntion {
    pub(crate) fn execute(&self, cv_image: &CvImage) -> Vec<dlib_support::face::Face> {
        self.face_recognition.get_faces(cv_image)
    }
}
