pub(crate) mod camera;
pub(crate) mod display;
pub(crate) mod face_manager;
pub(crate) mod face_recognition;
pub(crate) mod login;

use std::str::FromStr;

use crate::core::{camera::Camera, face_manager::FaceManager, login::Login};
use color_eyre::{eyre::Ok, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use display::Display;
use dlib_support::face::Face;
use dlib_sys::{
    cv_image::CvImage, face_recognition_resnet_model_v1::FaceRecognitionResnetModelV1,
    frontal_face_detector::FrontalFaceDetector, full_object_detection::FullObjectDetection, matrix::Matrix,
    matrix_descriptor::MatrixDescriptor, rectangle::Rectangle, shape_predictor::ShapePredictor,
};
use railwork::nodes::{produce::ProduceNode, transform::TransformNode};

pub(crate) enum OperationMode {
    Add,
    Login,
    Live,
}

#[derive(Default)]
pub struct ImageMessage {
    pub cv_image: Option<CvImage>,
    pub faces: Option<Vec<FaceMessage>>,
}

#[derive(Default)]
pub struct FaceMessage {
    pub rectangle: Option<Rectangle>,
    pub full_object_detection: Option<FullObjectDetection>,
    pub face_chip: Option<Matrix>,
    pub face_descriptor: Option<MatrixDescriptor>,
}

impl From<FaceMessage> for Face {
    fn from(value: FaceMessage) -> Self {
        Face {
            rectangle: value.rectangle.unwrap(),
            full_object_detection: value.full_object_detection.unwrap(),
            face_chip: value.face_chip.unwrap(),
            face_descriptor: value.face_descriptor.unwrap(),
        }
    }
}

type ImageSender = Sender<ImageMessage>;
type ImageReceiver = Receiver<ImageMessage>;

pub(crate) struct Core {
    operation_mode: OperationMode,

    _camera_node: ProduceNode,
    _frontal_face_detector_node1: TransformNode,
    _frontal_face_detector_node2: TransformNode,
    _frontal_face_detector_node3: TransformNode,
    _frontal_face_detector_node4: TransformNode,
    _frontal_face_detector_node5: TransformNode,
    _frontal_face_detector_node6: TransformNode,
    _shape_predictor_node: TransformNode,
    _face_recognition_node: TransformNode,

    receiver: Receiver<ImageMessage>,
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
        let _camera_node = ProduceNode::new(
            move || ImageMessage {
                cv_image: Some(camera.run()),
                ..Default::default()
            },
            sender1,
            true,
        );

        let (sender2, receiver2): (ImageSender, ImageReceiver) = unbounded();

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node1 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node2 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );
        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node3 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node4 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node4 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node5 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node6 = TransformNode::new(
            move |mut input| {
                let rectangles = frontal_face_detector.function_call(input.cv_image.as_ref().unwrap());
                let faces = rectangles
                    .into_iter()
                    .map(|x| FaceMessage {
                        rectangle: Some(x),
                        ..Default::default()
                    })
                    .collect();
                input.faces = Some(faces);
                input
            },
            receiver1.clone(),
            sender2.clone(),
            true,
            String::from_str("Frontal face detector")?,
        );

        let (sender3, receiver3): (ImageSender, ImageReceiver) = unbounded();

        let mut shape_predictor = ShapePredictor::new(config.models.shape_predictor.filepath.to_str().unwrap());
        let _shape_predictor_node = TransformNode::new(
            move |mut input| {
                input.faces.as_mut().unwrap().iter_mut().for_each(|x| {
                    x.full_object_detection = Some(
                        shape_predictor
                            .function_call(input.cv_image.as_ref().unwrap(), x.rectangle.as_ref().unwrap().clone()),
                    );
                    x.face_chip = Some(Matrix::new());
                    dlib_sys::extract_image_chip(
                        input.cv_image.as_ref().unwrap(),
                        &dlib_sys::get_face_chip_details(x.full_object_detection.as_ref().unwrap(), 150, 0.25),
                        x.face_chip.as_mut().unwrap(),
                    )
                });
                input
            },
            receiver2,
            sender3,
            true,
            String::from_str("Shape predictor")?,
        );

        let (sender4, receiver4): (ImageSender, ImageReceiver) = unbounded();

        let mut display = matches!(operation_mode, OperationMode::Live).then(Display::default);

        let mut face_recognition =
            FaceRecognitionResnetModelV1::new(config.models.face_recognition.filepath.to_str().unwrap());
        let _face_recognition_node = TransformNode::new(
            move |mut input| {
                input.faces.as_mut().unwrap().iter_mut().for_each(|x| {
                    x.face_descriptor = Some(face_recognition.function_call(x.face_chip.as_ref().unwrap()))
                });

                if let Some(ref mut display) = display {
                    display.clear();
                    display.display(input.cv_image.as_ref().unwrap());

                    input.faces.as_ref().unwrap().iter().for_each(|face| {
                        let overlays = face.full_object_detection.as_ref().unwrap().render_face_detections();
                        display.overlay(face.rectangle.as_ref().unwrap().clone(), overlays);
                        display.display_face(face.face_chip.as_ref().unwrap());
                    })
                }

                input
            },
            receiver3,
            sender4,
            true,
            String::from_str("Face recognition")?,
        );

        Ok(Self {
            operation_mode,
            _camera_node,
            _frontal_face_detector_node1,
            _frontal_face_detector_node2,
            _frontal_face_detector_node3,
            _frontal_face_detector_node4,
            _frontal_face_detector_node5,
            _frontal_face_detector_node6,
            _shape_predictor_node,
            _face_recognition_node,
            receiver: receiver4,
        })
    }

    pub(crate) fn add(&mut self) -> Result<()> {
        let mut batch = vec![];

        loop {
            let mut faces = self.receiver.recv().expect("Fail to receive").faces.unwrap();
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
        face_add.add_identity(batch.into_iter().map(Into::into).collect());

        Ok(())
    }

    pub(crate) fn login(&mut self) -> Result<()> {
        let login = Login::default();
        loop {
            if let Some(x) = login.run(
                self.receiver
                    .recv()
                    .expect("Fail to receive")
                    .faces
                    .unwrap()
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ) {
                log::info!("Found user id: {}", x);
                if let OperationMode::Login = self.operation_mode {
                    break;
                }
            }
        }

        Ok(())
    }
}
