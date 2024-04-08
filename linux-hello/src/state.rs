use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
    sync::RwLock,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct Face {
    pub(crate) vec: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Identity {
    pub(crate) name: String,
    pub(crate) faces: Vec<Face>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Data {
    pub(crate) identities: Vec<Identity>,
}

impl Data {
    pub(crate) fn new() -> Self {
        Self { identities: vec![] }
    }
}

pub(crate) static GLOBAL_DATA: state::InitCell<RwLock<Data>> = state::InitCell::new();

pub fn serialize(file_path: &str) {
    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open output file");
    let writer = BufWriter::new(output_file);

    let _ = serde_json::to_writer(writer, GLOBAL_DATA.get());
}

pub fn deserialize(file_path: &str) {
    let data: Data = if Path::new(file_path).exists() {
        let file = File::open(file_path).expect("Failed to read file");
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).expect("Failed to deserialize")
    } else {
        Data::new()
    };

    GLOBAL_DATA.set(RwLock::new(data));
}
