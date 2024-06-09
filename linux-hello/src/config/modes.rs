use super::{face_recognition::FaceRecognition, shape_predictor::ShapePredictor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Models {
    #[serde(default)]
    pub(crate) shape_predictor: ShapePredictor,

    #[serde(default)]
    pub(crate) face_recognition: FaceRecognition,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            shape_predictor: ShapePredictor::default(),
            face_recognition: FaceRecognition::default(),
        }
    }
}
