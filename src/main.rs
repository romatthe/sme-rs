use std::fs::File;
use std::io;
use std::io::Cursor;
use std::mem;

use binrw::{BinRead, binread, BinReaderExt, binrw, FilePtr, NullString};
use binrw::helpers::until_exclusive;

fn main() -> io::Result<()> {
    let mut file = File::open("appinfo.vdf")?;
    let app_info = AppInfo::read(&mut file).unwrap();

    // println!("Universe: {:#04x}", app_info.universe);
    println!("{} apps", app_info.apps.len());
    
    for app in app_info.apps  {
        // App: 2180100 - Token: 18040690077583379468 - Proton Hotfix
        // Console.WriteLine($"App: {app.AppID} - Token: {app.Token} - {app.Data["common"]["name"]}");
        // if app.pics_token > 0 {
            println!("App: {} - Token: {}", app.appid, app.pics_token);
        // }

        // println!("Appid: {}", app.appid);
        // println!("size: {}", app.size);
        // println!("SHA1: {:?}", app.sha1);
        // println!("Blob size: {}", app.blob.len());
    }

    // println!("EOF: {}", app_info.eof);

    println!("String table offset: {}", app_info.string_table_count_ptr.ptr);
    println!("String table ptr count: {}", *app_info.string_table_count_ptr);
    println!("String table count: {}", app_info.string_table_count);

    // for string in app_info.string_table  {
    //     println!("NullString: {}", string.to_string());
    // }

    Ok(())
}

#[binread]
#[br(magic = b"\x29\x44\x56\x07", little)]
struct AppInfo {
    universe: u32,
    
    // #[br(temp)]
    string_table_count_ptr: FilePtr<i64, u32>,
    
    #[br(parse_with = until_exclusive(|section: &AppSection| section.appid == 0))]
    apps: Vec<AppSection>,

    string_table_count: u32,

    #[br(count = string_table_count)]
    string_table: Vec<NullString>,
}

#[binread]
#[br(little)]
struct AppSection {
    appid: u32,

    #[br(if(appid != 0, 36))]
    size: u32, // -36?

    #[br(if(appid != 0))]
    info_state: u32,

    #[br(if(appid != 0))]
    last_updated: u32,

    #[br(if(appid != 0))]
    pics_token: u64,

    #[br(if(appid != 0))]
    sha1: [u8; 20],

    #[br(if(appid != 0), count = size - 36)]
    blob: Vec<u8>,
}