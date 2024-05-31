mod face_recognition;
mod modes;
mod shape_predictor;
mod video;

use self::{modes::Models, video::Video};
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf, sync::RwLock};

static ENV_PREFIX: &str = "LINUX_HELLO__";
static ETC_PATH: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| PathBuf::from("/etc/linux-hello/"));
static STATE_PATH: once_cell::sync::Lazy<PathBuf> =
    once_cell::sync::Lazy::new(|| PathBuf::from("/var/lib/linux-hello/"));

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
    fn new() -> Self {
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

pub(crate) static GLOBAL_CONFIG: once_cell::sync::Lazy<RwLock<Config>> =
    once_cell::sync::Lazy::new(|| RwLock::new(Config::new()));
