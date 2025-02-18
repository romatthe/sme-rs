use std::fs::File;
use std::io;
use std::io::Read;

use nom::Finish;
use sha1::digest::DynDigest;
use crate::appinfo::AppInfoParserPacker;

mod v29;
mod vdf;
mod appinfo;

fn main() -> anyhow::Result<()> {
    let file = File::open("appinfo.vdf")?;
    let app_info = v29::AppInfo::parse(file)?;

    // println!("Apps: {}", app_info.apps.len());

    let app = app_info.apps.iter().find(|app| app.appid == 1325200).unwrap();

    println!("Found: {:?}", app);

    for app in app_info.apps {
        // println!("AppID: {}", app.appid);
    }

    Ok(())
}
