use cxx::UniquePtr;

pub struct ChipDetails {
    pub(crate) inner: crate::ffi::wrapper::ChipDetails,
}
