mod camera;
mod face_recognition;

use self::{camera::Camera, face_recognition::FaceRecogntion};
use dlib_support::face::Face;
use railwork::{produce::ProduceBlock, transform::TransformBlock};
use std::{sync::mpsc::Receiver, time::Instant};

pub(crate) struct Core {
    pub(crate) receiver: Receiver<Vec<Face>>,
    camera_block: ProduceBlock,
    face_recognition_block: TransformBlock,
}

impl Default for Core {
    fn default() -> Self {
        let (sender1, receiver1) = std::sync::mpsc::channel();
        let (sender2, receiver2) = std::sync::mpsc::channel();

        let camera_block = ProduceBlock::new(Camera::default(), sender1);

        let instant = Instant::now();
        let face_recognition_block = TransformBlock::new(FaceRecogntion::default(), receiver1, sender2);
        log::trace!("elapsed time {}", instant.elapsed().as_millis());

        Self {
            camera_block,
            face_recognition_block,
            receiver: receiver2,
        }
    }
}
