use serde::{Deserialize, Serialize};
use std::sync::RwLock;

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
