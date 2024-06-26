use crate::face::Face;
use dlib_sys::{
    cv_image::CvImage, face_recognition_resnet_model_v1::FaceRecognitionResnetModelV1,
    frontal_face_detector::FrontalFaceDetector, full_object_detection::FullObjectDetection, matrix::Matrix,
    matrix_descriptor::MatrixDescriptor, rectangle::Rectangle, shape_predictor::ShapePredictor,
};
use std::path::Path;

pub struct FaceRecognition {
    frontal_face_detector: FrontalFaceDetector,
    shape_predictor: ShapePredictor,
    face_recognition_resnet_model_v1: FaceRecognitionResnetModelV1,
}

impl FaceRecognition {
    pub fn new<P: AsRef<Path>>(shape_predictor_file_path: P, face_recognition_file_path: P) -> Self {
        Self {
            frontal_face_detector: FrontalFaceDetector::new(),
            shape_predictor: ShapePredictor::new(shape_predictor_file_path.as_ref().to_str().unwrap()),
            face_recognition_resnet_model_v1: FaceRecognitionResnetModelV1::new(
                face_recognition_file_path.as_ref().to_str().unwrap(),
            ),
        }
    }

    pub fn get_faces(&mut self, cv_image: &CvImage) -> Vec<crate::face::Face> {
        let rectangles: Vec<dlib_sys::rectangle::Rectangle> = self.frontal_face_detector.function_call(cv_image);

        let faces: Vec<Face> = rectangles
            .into_iter()
            .map(|x| self.generate_face(cv_image, x))
            .collect();

        faces
    }

    fn generate_face(&mut self, cv_image: &CvImage, rectangle: Rectangle) -> Face {
        let mut image_chip: Matrix = Matrix::new();

        let full_object_detection: FullObjectDetection =
            self.shape_predictor.function_call(cv_image, rectangle.clone());
        dlib_sys::extract_image_chip(
            cv_image,
            &dlib_sys::get_face_chip_details(&full_object_detection, 150, 0.25),
            &mut image_chip,
        );

        let matrix_descriptor: MatrixDescriptor = self.face_recognition_resnet_model_v1.function_call(&image_chip);

        Face {
            rectangle: rectangle.clone(),
            full_object_detection,
            face_chip: image_chip,
            face_descriptor: matrix_descriptor,
        }
    }
}
