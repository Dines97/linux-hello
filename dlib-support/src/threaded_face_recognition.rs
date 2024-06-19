use crossbeam_channel::{unbounded, Receiver, Sender};
use dlib_sys::{
    cv_image::CvImage, face_recognition_resnet_model_v1::FaceRecognitionResnetModelV1,
    frontal_face_detector::FrontalFaceDetector, full_object_detection::FullObjectDetection, matrix::Matrix,
    matrix_descriptor::MatrixDescriptor, rectangle::Rectangle, shape_predictor::ShapePredictor,
};
use railwork::nodes::transform::TransformNode;
use std::path::Path;

pub struct FaceRecognition {
    sender: Sender<ImageMessage>,

    _frontal_face_detector_node: TransformNode,
    _shape_predictor_node: TransformNode,
    _face_recognition_node: TransformNode,

    receiver: Receiver<ImageMessage>,
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

type ImageSender = Sender<ImageMessage>;
type ImageReceiver = Receiver<ImageMessage>;

impl FaceRecognition {
    pub fn new<P: AsRef<Path>>(shape_predictor_file_path: P, face_recognition_file_path: P) -> Self {
        let (sender1, receiver1): (ImageSender, ImageReceiver) = unbounded();
        let (sender2, receiver2): (ImageSender, ImageReceiver) = unbounded();

        let mut frontal_face_detector = FrontalFaceDetector::new();
        let _frontal_face_detector_node = TransformNode::new(
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
            receiver1,
            sender2,
            false,
            String::new(),
        );

        let (sender3, receiver3): (ImageSender, ImageReceiver) = unbounded();

        let mut shape_predictor = ShapePredictor::new(shape_predictor_file_path.as_ref().to_str().unwrap());
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
            false,
            String::new(),
        );

        let (sender4, receiver4): (ImageSender, ImageReceiver) = unbounded();

        let mut face_recognition =
            FaceRecognitionResnetModelV1::new(face_recognition_file_path.as_ref().to_str().unwrap());
        let _face_recognition_node = TransformNode::new(
            move |mut input| {
                input.faces.as_mut().unwrap().iter_mut().for_each(|x| {
                    x.face_descriptor = Some(face_recognition.function_call(x.face_chip.as_ref().unwrap()))
                });

                input
            },
            receiver3,
            sender4,
            false,
            String::new(),
        );

        Self {
            sender: sender1,
            _frontal_face_detector_node,
            _shape_predictor_node,
            _face_recognition_node,
            receiver: receiver4,
        }
    }
}
