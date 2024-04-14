use clap::Args;

use crate::core::Core;

use super::Runnable;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct AddArgs {}

impl Runnable for AddArgs {
    fn run(&self) -> Result<()> {
        let mut core = Core::default();
        core.run()
    }
}

// impl Runnable for AddArgs {
//     fn run(&self) -> Result<()> {
//         let user = User::current();
//
//         let mut video_capture: VideoCapture = VideoCapture::new(0, VideoCaptureAPIs::CapAny);
//         let config = GLOBAL_CONFIG.read().unwrap();
//         let face_recognition: FaceRecognition = build_face_recognition();
//         let mut mat: Mat = Mat::default();
//
//         let mut faces: Vec<dlib_support::face::Face>;
//
//         loop {
//             // video_capture.stream_extraction(&mut mat);
//
//             // let cv_image: CvImage = CvImage::from(mat);
//             // faces = face_recognition.get_faces(&cv_image);
//
//             // if faces.len() == 1 {
//             //     break;
//             // }
//         }
//
//         let dlib_face = faces.first().unwrap();
//
//         let face: Face = Face {
//             vec: dlib_face.face_descriptor.to_vec(),
//         };
//
//         let identity: Identity = Identity {
//             name: String::from("placeholder"),
//             faces: vec![face],
//         };
//
//         log::info!("Writing user to database");
//         let a = GLOBAL_DATA.get().unwrap().write();
//         a.unwrap().identities.push(identity);
//
//         Ok(())
//     }
// }
