use super::{face_recognition::FaceRecognition, shape_predictor::ShapePredictor};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Models {
    #[serde(default)]
    pub(crate) dir: PathBuf,

    #[serde(default)]
    pub(crate) shape_predictor: ShapePredictor,

    #[serde(default)]
    pub(crate) face_recognition: FaceRecognition,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            dir: crate::data::DATA_DIR.join("linux-hello/models/"),
            shape_predictor: ShapePredictor::default(),
            face_recognition: FaceRecognition::default(),
        }
    }
}
