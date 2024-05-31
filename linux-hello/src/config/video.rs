use opencv_sys::video_capture::VideoCaptureAPIs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Video {
    pub(crate) camera_index: i32,
    pub(crate) video_capture_api: VideoCaptureAPIs,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            camera_index: 0,
            video_capture_api: VideoCaptureAPIs::CapAny,
        }
    }
}
