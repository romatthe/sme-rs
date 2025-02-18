use std::ffi::CString;
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
    pub(crate) table: Vec<CString>,
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
    pub(crate) sha1_text: Vec<u8>,      // TODO: REMOVE!
    pub(crate) change_number: u32,
    pub(crate) sha1_binary: Vec<u8>,    // TODO: REMOVE!
    pub(crate) vdf: Vec<VdfNode>,
}

impl AppInfoParserPacker for AppInfo {
    fn parse(app_info_file: File) -> anyhow::Result<Self> {
        let app_info = parse_app_info(app_info_file)?;

        Ok(app_info)
    }

    fn pack(self, mut app_info_file: File) -> anyhow::Result<()> {
        packer::pack_app_info(&mut app_info_file, &self)?;

        Ok(())
    }

    fn update_entry() -> anyhow::Result<()> {
        todo!()
    }
}