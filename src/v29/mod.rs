use std::fs::File;
use std::io::{BufReader, Read};

use crate::appinfo::AppInfoParserPacker;
use crate::v29::parser::parse_app_info;
use crate::vdf::parser::VdfNode;

pub(crate) mod parser;
pub(crate) mod packer;

pub const HEADER_VERSION: &[u8; 1] = b"\x29";
pub const HEADER_MAGIC: &[u8; 3] = b"\x44\x56\x07";

#[derive(Debug)]
pub struct AppInfo {
    pub(crate) header: AppInfoHeader,
    pub(crate) apps: Vec<AppSection>,
    pub(crate) table: Vec<String>,
}

#[derive(Debug)]
pub struct AppInfoHeader {
    pub(crate) universe: u32,
    pub(crate) offset: i64,
}

#[derive(Debug)]
pub struct AppSection {
    pub(crate) appid: u32,
    pub(crate) info_state: u32,
    pub(crate) last_updated: u32,
    pub(crate) pics_token: u64,
    pub(crate) change_number: u32,
    pub(crate) blob: Vec<u8>,
}

impl AppInfoParserPacker for AppInfo {
    fn parse(mut app_info_file: File) -> anyhow::Result<Self> {
        let app_info = parse_app_info(app_info_file)?;

        // let mut input = Vec::new();
        // let buff_size = buffer.read_to_end(&mut input)?;

        Ok(app_info)
    }

    fn pack(self) -> anyhow::Result<()> {
        todo!()
    }

    fn update_entry() -> anyhow::Result<()> {
        todo!()
    }
}