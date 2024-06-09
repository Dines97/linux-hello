use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ShapePredictor {
    pub(crate) filepath: PathBuf,
    pub(crate) url: String,
}

impl Default for ShapePredictor {
    fn default() -> Self {
        Self {
            filepath: crate::data::DATA_DIR.join("models/shape_predictor_68_face_landmarks_GTX.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/shape_predictor_68_face_landmarks_GTX.dat.bz2",
            ),
        }
    }
}
