use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ShapePredictor {
    pub(crate) file_name: PathBuf,
    pub(crate) url: String,
}

impl Default for ShapePredictor {
    fn default() -> Self {
        Self {
            file_name: PathBuf::from("shape_predictor_68_face_landmarks_GTX.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/shape_predictor_68_face_landmarks_GTX.dat.bz2",
            ),
        }
    }
}
