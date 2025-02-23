use std::ffi::CString;

pub(crate) mod parser;
pub(crate) mod packer;
pub(crate) mod serializer;


#[derive(Clone, Debug)]
pub struct VdfNode {
    key: VdfStringRef,
    value: VdfNodeKind,
}

#[derive(Clone, Debug)]
pub enum VdfNodeKind {
    Nested {
        /// This cannot be made info a hashmap, since some apps have duplicate key/values. Perhaps
        /// it's fine to remove these duplicate values, but I'm erring on the side of caution and just
        /// leaving the duplicate key/values in, as it makes it easier to validate the packing logic.
        nodes: Vec<VdfNode>,
    },
    String {
        value: String,
    },
    Int {
        value: u32,
    },
}

#[derive(Clone, Debug)]
pub struct VdfStringRef(pub u32);