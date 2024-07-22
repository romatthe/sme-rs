use std::fs::File;
use std::io;
use std::io::Cursor;

use binrw::{BinRead, BinReaderExt, binrw, FilePtr, NullString};
use binrw::helpers::until_exclusive;

fn main() -> io::Result<()> {
    let mut file = File::open("appinfo.vdf")?;
    let app_info = AppInfo::read(&mut file).unwrap();

    println!("Universe: {:#04x}", app_info.universe);
    println!("Ptr: {:#04x}", app_info.string_table_offset);
    println!("App section count: {}", app_info.apps.len());

    for app in app_info.apps  {
        println!("Appid: {}", app.appid);
        println!("size: {}", app.size);
        println!("SHA1: {:?}", app.sha1);
        println!("Blob size: {}", app.blob.len());
    }

    Ok(())
}

#[binrw]
#[brw(magic = b"\x29\x44\x56\x07", little)]
struct AppInfo {
    universe: u32,
    // string_table_ptr: FilePtr<i64, u8>
    string_table_offset: i64,
    #[br(parse_with = until_exclusive(|section: &AppSection| section.appid == 0))]
    apps: Vec<AppSection>,

    // etc...
}

#[binrw]
#[br(little)]
struct AppSection {
    appid: u32,
    size: u32, // -36?
    info_state: u32,
    last_updated: u32,
    pics_token: u64,
    sha1: [u8; 20],
    #[br(count = size - 36)]
    blob: Vec<u8>,
    // etc....
}