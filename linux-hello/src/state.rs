use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
    sync::RwLock,
};

use crate::config::GLOBAL_CONFIG;

#[derive(Serialize, Deserialize)]
pub(crate) struct Face {
    pub(crate) vec: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Identity {
    pub(crate) name: String,
    pub(crate) faces: Vec<Face>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) identities: Vec<Identity>,
}

pub(crate) static GLOBAL_DATA: state::InitCell<RwLock<Data>> = state::InitCell::new();

pub fn serialize() {
    let config = GLOBAL_CONFIG.read().unwrap();

    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config.state_path)
        .expect("Failed to open output file");
    let writer = BufWriter::new(output_file);

    let _ = serde_json::to_writer(writer, GLOBAL_DATA.get());
}

pub fn deserialize() {
    let config = GLOBAL_CONFIG.read().unwrap();

    let path = Path::new(&config.state_path);
    fs::create_dir_all(path.parent().expect("Wrong state path")).expect("Can't create folder for state file");

    let data: Data = if path.exists() {
        let file = File::open(path).expect("Failed to read file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to deserialize")
    } else {
        Data::default()
    };

    GLOBAL_DATA.set(RwLock::new(data));
}
