use std::fs::File;
use std::io;
use std::io::Read;
use std::str;

use nom::bytes::streaming::{tag, take_until};
use nom::bytes::streaming::take;
use nom::combinator::{eof, map_res};
use nom::{Finish, IResult};
use nom::multi::{count, many_till};
use nom::number::streaming::{le_i64, le_u32, le_u64};
use nom::sequence::terminated;
use sha1::digest::DynDigest;

mod v29;

fn main() -> io::Result<()> {
    let mut file = File::open("appinfo.vdf")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (_ , app_info) = parse_app_info(&buffer).expect("Hemel");

    println!("Apps: {}", app_info.apps.len());

    for app in app_info.apps {
        // println!("AppID: {}", app.appid);
    }

    Ok(())
}

fn parse_app_info(input: &[u8]) -> IResult<&[u8], AppInfo> {
    let (input, _) = tag(b"\x29\x44\x56\x07")(input)?;
    let (input, universe) = le_u32(input)?;
    let (input, table_offset) = le_i64(input)?;
    let (input, apps) = parse_app_sections(input)?;
    let (input, string_count) = le_u32(input)?;
    let (input, string_table) = count(parse_nullstring, string_count as usize)(input)?;
    let x = eof(input)?;

    println!("String table count: {}", string_count);
    println!("Vector count: {}", string_table.len());

    Ok((
        input,
        AppInfo {
            universe,
            apps,
        },
    ))
}

fn parse_app_sections(input: &[u8]) -> IResult<&[u8], Vec<AppSection>> {
    let (input, (sections, _)) = many_till(parse_app_section, tag(b"\0\0\0\0"))(input)?;

    println!("Next byte after parsing apps: {}", u32::from_le_bytes([input[0], input[1], input[2], input[3]]));

    Ok((
        input,
        sections,
    ))
}

fn parse_app_section(input: &[u8]) -> IResult<&[u8], AppSection> {
    // uint32   - AppID
    // uint32   - size // until end of binary_vdf
    // uint32   - infoState // mostly 2, sometimes 1 (may indicate prerelease or no info)
    // uint32   - lastUpdated
    // uint64   - picsToken
    // 20bytes  - SHA1 // of text appinfo vdf, as seen in CMsgClientPICSProductInfoResponse.AppInfo.sha
    // uint32   - changeNumber
    // 20bytes  - SHA1 // of binary_vdf
    // variable - binary_vdf
    let (input, appid) = le_u32(input)?;
    let (input, size) = le_u32(input)?;
    let (input, info_state) = le_u32(input)?;
    let (input, last_updated) = le_u32(input)?;
    let (input, pics_token) = le_u64(input)?;
    let (input, sha1) = take(20usize)(input)?;
    let (input, blob) = take(size - 36)(input)?;

    // println!("AppID: {}", appid);
    // println!("Size: {}", size);
    // println!("Size until: {}", size - 36);
    // println!("Info state: {}", info_state);
    // println!("Last updated: {}", last_updated);
    // println!("Pics token: {}", pics_token);
    // println!("Sha1: {:?}", sha1);
    // println!("Blob size: {}", blob.len());
    // println!("Next byte: {}", u32::from_le_bytes([input[0], input[1], input[2], input[3]]));

    Ok((
        input,
        AppSection {
            appid,
            blob: blob.into(),
            // string_table: string_table.into(),
        }
    ))
}

/// Parse a null-terminated variable length string.
fn parse_nullstring(input: &[u8]) -> IResult<&[u8], &str> {
    let null_str = terminated(take_until("\0"), tag("\0"));
    map_res(null_str,  |s|str::from_utf8(s))(input)
}

struct AppInfo {
    universe: u32,
    apps: Vec<AppSection>,
}

struct AppSection {
    appid: u32,
    blob: Vec<u8>,
    // string_table: Vec<String>
}
