use dlib::face_recognition::FaceRecognition;

use crate::config::GLOBAL_CONFIG;

pub(crate) fn build_face_recognition() -> FaceRecognition {
    let config = GLOBAL_CONFIG.read().unwrap();

    let face_recognition: FaceRecognition = FaceRecognition::new(
        config.models.models_dir.join(&config.models.shape_predictor.file_name),
        config.models.models_dir.join(&config.models.face_recognition.file_name),
    );

    face_recognition
}
