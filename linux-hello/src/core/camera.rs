use crate::{config::GLOBAL_CONFIG, cycle_controller::CycleController};
use dlib_sys::cv_image::CvImage;
use opencv_sys::video_capture::VideoCapture;
use railwork::produce::Produce;

pub(crate) struct Camera {
    video_capture: VideoCapture,
    cycle_controller: CycleController,
}

unsafe impl Send for Camera {}

impl Default for Camera {
    fn default() -> Self {
        let config = GLOBAL_CONFIG.read().unwrap();

        log::info!("Begining of camera thread");

        log::info!("Camera index: {}", config.video.camera_index);
        log::info!("Camera api: {}", config.video.video_capture_api);
        let video_capture = VideoCapture::new(config.video.camera_index, config.video.video_capture_api);
        log::info!("OpenCV backend name: {}", video_capture.get_backend_name());

        let cycle_controller: CycleController = CycleController::new();

        Self {
            video_capture,
            cycle_controller,
        }
    }
}

impl Produce for Camera {
    type Output = dlib_sys::cv_image::CvImage;

    fn run(&mut self) -> Self::Output {
        log::trace!("Reading camera");
        let mat = self.video_capture.record();

        // cycle_controller.throttle(10.0);
        log::trace!("{}", self.cycle_controller);
        self.cycle_controller.update();

        CvImage::from(mat)
    }
}
