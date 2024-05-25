pub(crate) mod camera;
pub(crate) mod display;
pub(crate) mod face_add;
pub(crate) mod face_login;
pub(crate) mod face_recognition;

use crate::{
    config::GLOBAL_CONFIG,
    core::{camera::Camera, face_add::FaceAdd, face_recognition::FaceRecognition},
};
use crossbeam_channel::unbounded;
use railwork::{
    action::ActionBlock, batch::BatchBlock, filter::FilterBlock, produce::ProduceBlock, sync::SyncActionBlock, transform::TransformBlock
};

pub(crate) enum OperationMode {
    Add,
    Login,
}

pub(crate) struct Core {}

impl Core {
    pub(crate) fn new(camera_index: Option<i32>, enable_display: bool, operation_mode: OperationMode) -> Self {
        let config = GLOBAL_CONFIG.read().unwrap();

        // Camera to face recognition
        let (sender1, receiver1) = unbounded();

        // Read camera
        let camera_block = ProduceBlock::new(
            Camera::new(
                camera_index.unwrap_or(config.video.camera_index),
                config.video.video_capture_api,
            ),
            sender1,
        );

        // Face recognition to ?
        let (sender2, receiver2) = unbounded();

        // Perform face recognition
        let face_recognition_block =
            TransformBlock::from_transform(FaceRecognition::new(enable_display), receiver1, sender2);

        match operation_mode {
            OperationMode::Add => {
                // Filter to filter?
                let (sender3, receiver3) = unbounded();

                // Filter out multi face images
                let filter_block = TransformBlock::from_closure(
                    |mut x| match x.len() {
                        0 => None,
                        1 => x.pop(),
                        _ => panic!("Multiple faces detected on one of the images"),
                    },
                    receiver2,
                    sender3,
                );

                // Filter to batch
                let (sender4, receiver4) = unbounded();
                let filter = FilterBlock::new(receiver3, sender4);

                // Batch to add
                let (sender5, receiver5) = unbounded();
                let batch_block = BatchBlock::new_sized(128, receiver4, sender5);

                let face_add = SyncActionBlock::new(FaceAdd::default(), receiver5);
            }
            OperationMode::Login => todo!(),
        }

        Self {}
    }
}
