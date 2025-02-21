use std::fs::File;
use anyhow::Result;

pub trait AppInfoParserPacker {
    fn parse(app_info_file: File) -> Result<Self> where Self: Sized;
    fn pack(self, app_info_file: File) -> Result<()>;
    fn patch_app(&mut self, patch: AppPatch) -> Result<()>;
}

pub struct AppPatch {
    pub appid: u32,
    pub name: String,
    pub sort_as: Option<String>,
}