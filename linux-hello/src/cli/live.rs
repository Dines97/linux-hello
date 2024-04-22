use std::time::Duration;

use crate::core::Core;
use crate::Runnable;

use clap::Args;
use color_eyre::Result;

#[derive(Debug, Args)]
pub(crate) struct LiveArgs {
    #[arg(short, long)]
    pub camera: Option<i32>,
}

impl Runnable for LiveArgs {
    fn run(&self) -> Result<()> {
        let mut core = Core::default();
        // core.run()

        std::thread::sleep(Duration::new(60, 0));
        Ok(())
    }
}

// impl Runnable for LiveArgs {
//     fn run(&self) -> Result<()> {
//         let config = GLOBAL_CONFIG.read().unwrap();
//
//         let mut image_window: ImageWindow = ImageWindow::new();
//         let mut face_image_window: ImageWindow = ImageWindow::new();
//
//         let mut cycle_controller: CycleController = CycleController::new();
//
//         let mut prev: Vec<f32> = vec![0_f32; 128];
//
//         let mut camera_face_recognition =
//             face_recognition::CameraFaceRecogntion::new(config.video.camera_index, config.video.video_capture_api);
//
//         'main: loop {
//             let camera_face_recognition_result = camera_face_recognition.execute();
//             image_window.set_cv_image(&camera_face_recognition_result.cv_image);
//
//             camera_face_recognition_result.faces.iter().for_each(|x| {
//                 image_window.add_rectangle_overlay(x.rectangle.clone());
//                 let overlays = x.full_object_detection.render_face_detections();
//                 image_window.add_line_overlay(overlays);
//                 face_image_window.set_matrix(&x.face_chip);
//
//                 euclidean_distance(&x.face_descriptor.to_vec(), &prev.to_vec());
//
//                 // log::trace!(
//                 //     "Accuracy: {}",
//                 //     euclidean_distance(&x.face_descriptor.to_vec(), &prev.to_vec())
//                 // );
//
//                 prev = x.face_descriptor.to_vec();
//             });
//
//             // cycle_controller.throttle(10.0);
//             log::trace!("{}", cycle_controller);
//             cycle_controller.update();
//
//             if image_window.is_closed() {
//                 break 'main;
//             }
//
//             image_window.clear_overlay();
//         }
//
//         Ok(())
//     }
// }
