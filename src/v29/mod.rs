use std::ffi::CString;
use std::fs::File;
use std::io::{BufReader, Read};
use indexmap::IndexMap;
use crate::appinfo::{AppInfoParserPacker, AppPatch};
use crate::v29::parser::parse_app_info;
use crate::vdf::{VdfNode, VdfNodeKind, VdfStringRef};

pub(crate) mod parser;
pub(crate) mod packer;

pub const HEADER_VERSION: &[u8; 1] = b"\x29";
pub const HEADER_MAGIC: &[u8; 3] = b"\x44\x56\x07";

#[derive(Debug)]
pub struct AppInfo {
    pub(crate) header: AppInfoHeader,
    pub(crate) apps: IndexMap<u32, AppSection>, // Indexed by AppID
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
    pub(crate) sha1_text: Vec<u8>,      // TODO: REMOVE!
    pub(crate) change_number: u32,
    pub(crate) sha1_binary: Vec<u8>,    // TODO: REMOVE!
    pub(crate) vdf: VdfNode,
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
        if let Some(app) = self.apps.get_mut(&patch.appid) {
            // Set the `name` value in the VDF
            let mut name = app.vdf
                .get_mut("common")
                .and_then(|n| n.get_mut("name"));

            if let Some(VdfNode { key, value: VdfNodeKind::String { ref mut value } }) = &mut name {
                *value = patch.name.clone();
            }

            // Some games have a `name` tag in a non-English language, so the *localized* English
            // name has to be changed as well
            let mut name_localized = app.vdf
                .get_mut("common")
                .and_then(|n| n.get_mut("name_localized"))
                .and_then(|n| n.get_mut("english"));

            if let Some(VdfNode { key, value: VdfNodeKind::String { ref mut value } }) = &mut name_localized {
                *value = patch.name.clone();
            }

            // let mut sort_as = app.vdf
            //     .get_mut("common")
            //     .and_then(|n| n.get_mut("sortas"));
            //
            // // Get the existing `sortas` node and set it to either a new sortas value or just the name
            // if let Some(VdfNode { key, value: VdfNodeKind::String { ref mut value } }) = &mut sort_as {
            //     if let Some(sort_as_val) = patch.sort_as {
            //         *value = sort_as_val;
            //     } else {
            //         *value = patch.name;
            //     }
            // } else {
            //     // If no `sortas` node exists, but we are trying to change the node, we need to add it as the last
            //     // element in the `common` list.
            //     let mut common = app.vdf.get_mut("common");
            //     if let (Some(VdfNode{ key, value:  VdfNodeKind::Nested { nodes }}), Some(sort_as_val)) = (common, patch.sort_as) {
            //         // First we need to know the StringRef that's used in the string table for the `sortas` label
            //         let (ref_id, _) = self.table.iter().enumerate().find(|(i, &ref s)| s == "sortas").unwrap(); // TODO: unwrap!!!! What if it's not in the table?
            //         nodes.push(VdfNode {
            //             key: VdfStringRef { string_ref: ref_id as u32, string: Some("sortas".to_string()) },
            //             value: VdfNodeKind::String { value: sort_as_val }
            //         })
            //     }
            // }
        }

        Ok(())
    }
}

impl AppSection {
    pub fn complete_string_refs(&mut self, string_table: &[String]) {
        self.vdf.complete_string_refs(string_table)
    }
}