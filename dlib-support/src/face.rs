pub struct Face {
    pub rectangle: dlib_sys::rectangle::Rectangle,
    pub full_object_detection: dlib_sys::full_object_detection::FullObjectDetection,
    pub face_chip: dlib_sys::matrix::Matrix,
    pub face_descriptor: dlib_sys::matrix_descriptor::MatrixDescriptor,
}

unsafe impl Send for Face {}
