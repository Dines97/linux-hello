use std::{env, path::PathBuf, sync::RwLock};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use opencv_sys::video_capture::VideoCaptureAPIs;
use serde::{Deserialize, Serialize};

static ENV_PREFIX: &str = "LINUX_HELLO__";
static ETC_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/etc/linux-hello/"));
static STATE_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/var/lib/linux-hello/"));

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ShapePredictor {
    pub(crate) file_name: PathBuf,
    pub(crate) url: String,
}

impl Default for ShapePredictor {
    fn default() -> Self {
        Self {
            file_name: PathBuf::from("shape_predictor_68_face_landmarks_GTX.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/shape_predictor_68_face_landmarks_GTX.dat.bz2",
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FaceRecognition {
    pub(crate) file_name: PathBuf,
    pub(crate) url: String,
}

impl Default for FaceRecognition {
    fn default() -> Self {
        Self {
            file_name: PathBuf::from("dlib_face_recognition_resnet_model_v1.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/dlib_face_recognition_resnet_model_v1.dat.bz2",
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Models {
    #[serde(default)]
    pub(crate) models_dir: PathBuf,

    #[serde(default)]
    pub(crate) shape_predictor: ShapePredictor,

    #[serde(default)]
    pub(crate) face_recognition: FaceRecognition,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            models_dir: STATE_PATH.join("linux-hello/models/"),
            shape_predictor: ShapePredictor::default(),
            face_recognition: FaceRecognition::default(),
        }
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) models: Models,

    #[serde(default)]
    pub(super) state_path: PathBuf,

    #[serde(default)]
    pub(crate) video: Video,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            models: Models::default(),
            state_path: STATE_PATH.join("linux-hello/state.json"),
            video: Video::default(),
        }
    }
}

impl Config {
    pub(crate) fn new() -> Self {
        let config_path = match env::var(String::from(ENV_PREFIX) + "CONFIG_PATH") {
            Ok(x) => PathBuf::from(&x),
            Err(_) => ETC_PATH.join("config.toml"),
        };

        if !config_path.exists() {
            log::warn!("Config path invalid")
        }

        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_path))
            .merge(Env::prefixed(ENV_PREFIX).split("__"))
            .extract()
            .expect("Unable to build configuration");

        log::debug!("Printing out configurartion");
        log::debug!("{:?}", config);

        config
    }
}

pub(crate) static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::new()));
