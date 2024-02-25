use cxx::CxxVector;

pub struct Rectangle {
    pub(crate) inner: cxx::UniquePtr<crate::ffi::wrapper::Rectangle>,
}

// NOTE: Find a way to deal with `UniquePtr<CxxVector<crate::ffi::Wrapper::Rectangle>>` and remove
// this struct
pub struct Rectangles {
    pub(crate) inner: cxx::UniquePtr<CxxVector<crate::ffi::wrapper::Rectangle>>,
}
