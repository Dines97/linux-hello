use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Face {
    pub(crate) vec: Vec<f32>,
}
