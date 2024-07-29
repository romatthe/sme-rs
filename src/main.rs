use std::fs::File;
use std::io;
use std::io::Read;

use nom::Finish;
use sha1::digest::DynDigest;

mod v29;
mod vdf;

fn main() -> io::Result<()> {
    let mut file = File::open("appinfo.vdf")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (_ , app_info) = v29::parser::parse_app_info(&buffer).unwrap();

    println!("Apps: {}", app_info.apps.len());

    for app in app_info.apps {
        // println!("AppID: {}", app.appid);
    }

    Ok(())
}
