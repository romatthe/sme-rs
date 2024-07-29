pub(crate) mod parser;
pub(crate) mod packer;

pub const HEADER_VERSION: &[u8; 1] = b"\x29";
pub const HEADER_MAGIC: &[u8; 3] = b"\x44\x56\x07";