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

static CONFIG_DIR: once_cell::sync::Lazy<PathBuf> = once_cell::sync::Lazy::new(|| PathBuf::from("/etc/linux-hello/"));

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) models: Models,

    #[serde(default)]
    pub(super) data_filepath: PathBuf,

    #[serde(default)]
    pub(crate) video: Video,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            models: Models::default(),
            data_filepath: crate::data::DATA_DIR.join("data.json"),
            video: Video::default(),
        }
    }
}

impl Config {
    fn new() -> Self {
        let config_filepath = match env::var(String::from(ENV_PREFIX) + "CONFIG_FILEPATH") {
            Ok(x) => PathBuf::from(&x),
            Err(_) => CONFIG_DIR.join("config.toml"),
        };

        if !config_filepath.exists() {
            log::warn!("Config path invalid")
        }

        let config: Config = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_filepath))
            .merge(Env::prefixed(ENV_PREFIX).split("__"))
            .extract()
            .expect("Unable to build configuration");

        log::debug!("Printing out configurartion");
        log::debug!("{:?}", config);

        config
    }
}

static GLOBAL_CONFIG: once_cell::sync::Lazy<RwLock<Config>> = once_cell::sync::Lazy::new(|| RwLock::new(Config::new()));

pub(crate) fn read<'a>() -> std::sync::RwLockReadGuard<'a, Config> {
    GLOBAL_CONFIG.read().expect("Failed to read global config")
}
