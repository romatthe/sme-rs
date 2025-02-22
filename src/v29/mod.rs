use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, Read};
use indexmap::IndexMap;
use crate::appinfo::{AppInfoParserPacker, AppPatch};
use crate::v29::parser::parse_app_info;
use crate::vdf::{VdfNode, VdfStringRef};

pub(crate) mod parser;
pub(crate) mod packer;

pub const HEADER_VERSION: &[u8; 1] = b"\x29";
pub const HEADER_MAGIC: &[u8; 3] = b"\x44\x56\x07";

#[derive(Debug)]
pub struct AppInfo {
    pub(crate) header: AppInfoHeader,
    pub(crate) apps: IndexMap<u32, AppSection>, // Indexed by AppID
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
    pub(crate) vdf: Vec<(VdfStringRef, VdfNode)>,
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

    fn patch_app(&mut self, patch: AppPatch) -> anyhow::Result<()> {
        // match self.apps.get_mut(&patch.appid) {
        //     Some(app) => {
        //         if let VdfNode::Nested { key, nodes } = &mut app.vdf[0] {
        //             if let VdfNode::Nested { key, nodes } = &mut nodes[1] {
        //                 if let VdfNode::String { key: VdfString::StringRef(key), .. } = &mut nodes[0] {
        //                     nodes[0] = VdfNode::String { key: VdfString::StringRef(*key), value: VdfString::String(CString::new(patch.name)?) };
        //                     println!("{:?}", &nodes[0]);
        //                 }
        //             }
        //         }
        //     },
        //     None => {
        //         println!("Trying to patch App with AppID {}, but no entry found in `appinfo.vdf`", patch.appid);
        //     }
        // }

        Ok(())
    }
}