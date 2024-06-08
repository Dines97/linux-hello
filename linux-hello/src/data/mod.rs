pub(crate) mod face;
pub(crate) mod identity;
pub(crate) mod user;

use self::user::User;
use crate::config::GLOBAL_CONFIG;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::RwLock,
};

pub(crate) static DATA_DIR: once_cell::sync::Lazy<PathBuf> =
    once_cell::sync::Lazy::new(|| PathBuf::from("/var/lib/linux-hello/"));

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Data {
    pub users: Vec<User>,
}

impl Data {
    fn desialize() -> Self {
        let config = GLOBAL_CONFIG.read().unwrap();

        let path = Path::new(&config.data_filepath);
        fs::create_dir_all(path.parent().expect("Wrong state path")).expect("Can't create folder for state file");

        let data: Data = if path.exists() {
            let file = File::open(path).expect("Failed to read file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("Failed to deserialize")
        } else {
            Data::default()
        };

        data
    }

    pub(crate) fn serialize(&self) {
        let config = GLOBAL_CONFIG.read().unwrap();

        let output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&config.data_filepath)
            .expect("Failed to open output file");
        let writer = BufWriter::new(output_file);

        let _ = serde_json::to_writer(writer, self);
    }
}

pub(crate) static GLOBAL_DATA: once_cell::sync::Lazy<RwLock<Data>> =
    once_cell::sync::Lazy::new(|| RwLock::new(Data::desialize()));
