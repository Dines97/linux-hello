use dlib_sys::{
    cv_image::CvImage, frontal_face_detector::FrontalFaceDetector, rectangle::Rectangle,
    shape_predictor::ShapePredictor,
};

use crate::face::Face;

pub struct FaceRecognition {
    frontal_face_detector: FrontalFaceDetector,
    shape_predictor: ShapePredictor,
}

impl FaceRecognition {
    pub fn new() -> Self {
        Self {
            frontal_face_detector: FrontalFaceDetector::new(),
            shape_predictor: ShapePredictor::new(),
        }
    }

    pub fn get_faces(&self, cv_image: &mut CvImage) -> Vec<crate::face::Face> {
        let rectangles: Vec<dlib_sys::rectangle::Rectangle> =
            self.frontal_face_detector.function_call(cv_image);

        let faces: Vec<Face> = rectangles
            .into_iter()
            .map(|x| self.generate_face(cv_image, x))
            .collect();

        faces
    }

    fn generate_face(&self, cv_image: &mut CvImage, rectangle: Rectangle) -> Face {
        Face {
            rectangle: rectangle.clone(),
            full_object_detection: self.shape_predictor.function_call(cv_image, rectangle),
        }
    }
}
