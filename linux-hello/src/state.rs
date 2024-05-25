use crate::config::GLOBAL_CONFIG;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, BufWriter},
    path::Path,
    sync::RwLock,
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Face {
    pub(crate) vec: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Identity {
    pub(crate) name: String,
    pub(crate) faces: Vec<Face>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) name: String,
    pub(crate) uid: u32,
    pub identities: Vec<Identity>,
}

impl From<nix::unistd::User> for User {
    fn from(value: nix::unistd::User) -> Self {
        Self {
            name: value.name,
            uid: value.uid.into(),
            identities: vec![],
        }
    }
}

impl User {
    pub(crate) fn current() -> Self {
        let uid = nix::unistd::Uid::current();
        Self::from(nix::unistd::User::from_uid(uid).unwrap().unwrap())
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Data {
    pub users: Vec<User>,
}

pub(crate) static GLOBAL_DATA: once_cell::sync::OnceCell<RwLock<Data>> = once_cell::sync::OnceCell::new();

pub fn serialize() {
    let config = GLOBAL_CONFIG.read().unwrap();

    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config.state_path)
        .expect("Failed to open output file");
    let writer = BufWriter::new(output_file);

    let _ = serde_json::to_writer(writer, GLOBAL_DATA.get().unwrap());
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

    GLOBAL_DATA.set(RwLock::new(data)).unwrap();
}
