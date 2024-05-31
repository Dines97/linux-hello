use super::face::Face;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Identity {
    pub(crate) name: String,
    pub(crate) faces: Vec<Face>,
}
