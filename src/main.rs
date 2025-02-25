use crate::appinfo::{AppInfoParserPacker, AppPatch};

use sha1::Digest;
use std::fs::File;
use std::io::Read;

use nom::Finish;

mod v29;
mod vdf;
mod appinfo;

fn main() -> anyhow::Result<()> {
    // Find the configuration file
    let cleaner_dirs = xdg::BaseDirectories::with_prefix("steam-cleaner")?;
    let cleaner_config = cleaner_dirs.find_config_file("metadata.json").unwrap();

    // Find the Steam files
    let steam_dirs = xdg::BaseDirectories::with_prefix("Steam")?;
    let steam_vdf = steam_dirs.find_data_file("appcache/appinfo.vdf").unwrap();

    let file_patches = File::open(&cleaner_config)?;
    let file_appinfo = File::open(&steam_vdf)?;

    let patches: Vec<AppPatch> = serde_json::from_reader(file_patches)?;
    let mut app_info = v29::AppInfo::parse(file_appinfo)?;

    // Apply all known patches
    for patch in patches {
        app_info.patch_app(patch)?
    }

    // Write back the changes to `appinfo.vdf`
    let mut file_create = File::create(&steam_vdf)?;
    app_info.pack(file_create)?;

    Ok(())
}