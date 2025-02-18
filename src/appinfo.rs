use std::fs::File;
use anyhow::Result;

pub trait AppInfoParserPacker {
    fn parse(app_info_file: File) -> Result<Self> where Self: Sized;
    fn pack(self, app_info_file: File) -> Result<()>;
    fn update_entry() -> Result<()>;
}