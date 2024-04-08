use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, RwLock},
};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use once_cell::sync::{Lazy, OnceCell};
use opencv_sys::video_capture::VideoCaptureAPIs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ShapePredictor {
    pub(crate) file_path: String,

    pub(crate) url: String,
}

impl Default for ShapePredictor {
    fn default() -> Self {
        Self {
            file_path: String::from("/var/lib/linux-hello/models/shape_predictor_68_face_landmarks_GTX.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/shape_predictor_68_face_landmarks_GTX.dat.bz2",
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FaceRecognition {
    pub(crate) file_path: String,

    pub(crate) url: String,
}

impl Default for FaceRecognition {
    fn default() -> Self {
        Self {
            file_path: String::from("/var/lib/linux-hello/models/dlib_face_recognition_resnet_model_v1.dat"),
            url: String::from(
                "https://github.com/davisking/dlib-models/raw/master/dlib_face_recognition_resnet_model_v1.dat.bz2",
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Models {
    #[serde(default)]
    pub(crate) models_dir: String,

    #[serde(default)]
    pub(crate) shape_predictor: ShapePredictor,

    #[serde(default)]
    pub(crate) face_recognition: FaceRecognition,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            models_dir: String::from("/var/lib/linux-hello/models"),
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
    pub(super) state_path: String,

    #[serde(default)]
    pub(crate) video: Video,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            models: Models::default(),
            state_path: String::from("/var/lib/linux-hello/state.json"),
            video: Video::default(),
        }
    }
}

impl Config {
    pub(crate) fn new() -> Self {
        let config_file =
            Toml::file(&env::var("LINUX_HELLO__CONFIG_PATH").unwrap_or(String::from("/etc/linux-hello/config.toml")));

        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(config_file)
            .merge(Env::prefixed("LINUX_HELLO__").split("__"))
            .extract()
            .expect("Unable to build configuration");

        // let config_file = config::File::with_name(
        // );
        //
        // let config = config::Config::builder()
        //     .add_source(config_file)
        //     .add_source(
        //         config::Environment::with_prefix("LINUX_HELLO")
        //             .prefix_separator("_")
        //             .separator("_")
        //             .keep_prefix(false)
        //             .try_parsing(true)
        //             .convert_case(config::Case::Upper),
        //     )
        //     .build()
        //     .unwrap();
        //
        // log::debug!("{:?}", config.clone());
        //
        // let linux_hello_config: LinuxHelloConfig = config.try_deserialize().unwrap();

        log::debug!("Printing out configurartion");
        log::debug!("{:?}", config);

        return config;
    }
}

pub(crate) static GLOBAL_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::new()));

// pub(crate) static GLOBAL_CONFIG: state::InitCell<Config> = state::InitCell::new();
