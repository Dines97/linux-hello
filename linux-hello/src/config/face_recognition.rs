use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FaceRecognition {
    pub(crate) file_name: PathBuf,
    pub(crate) url: String,
}

impl Default for FaceRecognition {
    fn default() -> Self {
        Self {
            file_name: PathBuf::from("dlib_face_recognition_resnet_model_v1.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/dlib_face_recognition_resnet_model_v1.dat.bz2",
            ),
        }
    }
}
