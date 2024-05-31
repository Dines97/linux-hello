use super::{face_recognition::FaceRecognition, shape_predictor::ShapePredictor, STATE_PATH};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Models {
    #[serde(default)]
    pub(crate) models_dir: PathBuf,

    #[serde(default)]
    pub(crate) shape_predictor: ShapePredictor,

    #[serde(default)]
    pub(crate) face_recognition: FaceRecognition,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            models_dir: STATE_PATH.join("linux-hello/models/"),
            shape_predictor: ShapePredictor::default(),
            face_recognition: FaceRecognition::default(),
        }
    }
}
