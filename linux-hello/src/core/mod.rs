pub(crate) mod camera;
pub(crate) mod display;
pub(crate) mod face_manager;
pub(crate) mod face_recognition;
pub(crate) mod login;

use crate::core::{camera::Camera, face_manager::FaceManager, face_recognition::FaceRecognition, login::Login};
use color_eyre::{eyre::Ok, Result};
use crossbeam_channel::unbounded;
use railwork::nodes::{produce::ProduceNode, transform::TransformNode};

pub(crate) enum OperationMode {
    Add,
    Login,
    Live,
}

pub(crate) struct Core {
    operation_mode: OperationMode,

    _camera_node: ProduceNode,
    _face_recognition_node: TransformNode,

    receiver: crossbeam_channel::Receiver<Vec<dlib_support::face::Face>>,
}

impl Core {
    pub(crate) fn new(camera_index: Option<i32>, operation_mode: OperationMode) -> Result<Self> {
        let config = crate::config::read();

        opencv_sys::set_num_threads(1);

        // Camera to face recognition
        let (sender1, receiver1) = unbounded();

        // Read camera
        let mut camera = Camera::new(
            camera_index.unwrap_or(config.video.camera_index),
            config.video.video_capture_api,
        );
        let _camera_node = ProduceNode::new(move || camera.run(), sender1);

        // Face recognition to ?
        let (sender2, receiver2) = unbounded();

        // Perform face recognition
        let mut face_recognition = FaceRecognition::new(matches!(operation_mode, OperationMode::Live))?;
        let _face_recognition_node = TransformNode::new(move |x| face_recognition.run(x), receiver1, sender2);

        Ok(Self {
            operation_mode,
            _camera_node,
            _face_recognition_node,
            receiver: receiver2,
        })
    }

    pub(crate) fn add(&mut self) -> Result<()> {
        let mut batch = vec![];

        loop {
            let mut faces = self.receiver.recv().expect("Fail to receive");
            match faces.len() {
                0 => continue,
                1 => batch.push(faces.pop().unwrap()),
                _ => panic!("Multiple faces detected on one of the images"),
            };

            if batch.len() == 128 {
                break;
            }
        }

        let mut face_add = FaceManager::default();
        face_add.add_identity(batch);

        Ok(())
    }

    pub(crate) fn login(&mut self) -> Result<()> {
        loop {
            let live = Login::default();
            if let Some(x) = live.run(self.receiver.recv().expect("Fail to receive")) {
                log::info!("Found user id: {}", x);
                if let OperationMode::Login = self.operation_mode {
                    break;
                }
            }
        }

        Ok(())
    }
}
