use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread,
};

use dlib_sys::cv_image::CvImage;
use opencv_sys::video_capture::VideoCapture;

use crate::{config::GLOBAL_CONFIG, cycle_controller::CycleController};

pub(crate) struct VideoCaptureThread {
    handle: Option<thread::JoinHandle<()>>,
    continue_flag: Arc<Mutex<bool>>,
}

impl Drop for VideoCaptureThread {
    fn drop(&mut self) {
        log::info!("Drop video capture");

        log::info!("Stoping video capture thread");
        *self.continue_flag.lock().unwrap() = false;

        if let Some(handle) = self.handle.take() {
            handle.join().expect("Unable to join thread");
            log::info!("Joined camera thread");
        }
    }
}

impl VideoCaptureThread {
    // TODO: Learn if it correct usage of thread and arc
    pub(crate) fn new(sender: Sender<dlib_sys::cv_image::CvImage>) -> Self {
        let continue_flag = Arc::new(Mutex::new(true));
        let continue_flag_clone = Arc::clone(&continue_flag);

        let handler = Some(thread::spawn(move || {
            let config = GLOBAL_CONFIG.read().unwrap();

            log::info!("Begining of camera thread");

            log::info!("Camera index: {}", config.video.camera_index);
            log::info!("Camera api: {}", config.video.video_capture_api);
            let mut video_capture = VideoCapture::new(config.video.camera_index, config.video.video_capture_api);
            log::info!("OpenCV backend name: {}", video_capture.get_backend_name());

            let mut cycle_controller: CycleController = CycleController::new();

            while *continue_flag_clone.lock().unwrap() {
                log::trace!("Reading camera");
                let mat = video_capture.record();
                let cv_image: CvImage = CvImage::from(mat);

                let _ = sender.send(cv_image);

                // cycle_controller.throttle(10.0);
                log::trace!("{}", cycle_controller);
                cycle_controller.update();
            }
            log::info!("Ending camera loop");
        }));

        Self {
            handle: handler,
            continue_flag,
        }
    }
}
