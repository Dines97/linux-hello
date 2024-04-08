use clap::Args;
use dlib::face_recognition::FaceRecognition;
use dlib_sys::cv_image::CvImage;
use opencv_sys::{
    mat::Mat,
    video_capture::{VideoCapture, VideoCaptureAPIs},
};

use crate::{
    config::GLOBAL_CONFIG,
    state::{Face, Identity, GLOBAL_DATA},
};

use super::Runnable;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct AddArgs {}

impl Runnable for AddArgs {
    fn run(&self) -> Result<()> {
        let mut video_capture: VideoCapture = VideoCapture::new(0, VideoCaptureAPIs::CapAny);
        let config = GLOBAL_CONFIG.read().unwrap();
        let face_recognition: FaceRecognition = FaceRecognition::new(
            config.models.shape_predictor.file_path.clone(),
            config.models.face_recognition.file_path.clone(),
        );

        let mut mat: Mat = Mat::new();

        let mut faces: Vec<dlib::face::Face>;

        loop {
            video_capture.stream_extraction(&mut mat);

            let cv_image: CvImage = CvImage::new(&mat);
            faces = face_recognition.get_faces(&cv_image);

            if faces.len() == 1 {
                break;
            }
        }

        let dlib_face = faces.first().unwrap();

        let face: Face = Face {
            vec: dlib_face.face_descriptor.to_vec(),
        };

        let identity: Identity = Identity {
            name: String::from("placeholder"),
            faces: vec![face],
        };

        let a = GLOBAL_DATA.get().write();
        a.unwrap().identities.push(identity);

        Ok(())
    }
}
