use dlib_sys::cv_image::CvImage;
use opencv_sys::video_capture::{VideoCapture, VideoCaptureAPIs};

pub(crate) struct Camera {
    video_capture: VideoCapture,
}

unsafe impl Send for Camera {}

impl Default for Camera {
    fn default() -> Self {
        let config = crate::config::read();

        Self::new(config.video.camera_index, config.video.video_capture_api)
    }
}

impl Camera {
    pub(crate) fn new(camera_index: i32, api_preference: VideoCaptureAPIs) -> Self {
        log::info!("Begining of camera thread");

        log::info!("Camera index: {}", camera_index);
        log::info!("Camera api: {}", api_preference);
        let video_capture = VideoCapture::new(camera_index, api_preference);
        log::info!("OpenCV backend name: {}", video_capture.get_backend_name());

        Self { video_capture }
    }

    pub(crate) fn run(&mut self) -> CvImage {
        let mat = self.video_capture.record();

        CvImage::from(mat)
    }
}
