use nom::bytes::complete::{tag, take, take_until};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::{count, many_till};
use nom::number::complete::{le_i64, le_u32, le_u64};
use nom::sequence::{terminated, tuple};

use crate::v29::{HEADER_MAGIC, HEADER_VERSION};

#[derive(Debug)]
pub struct AppInfo<'a> {
    pub(crate) header: AppInfoHeader,
    pub(crate) apps: Vec<AppSection>,
    pub(crate) table: Vec<&'a str>,
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
    pub(crate) change_number: u32,
    pub(crate) blob: Vec<u8>,
}

/// Parse an `appinfo.vdf` file according to the v29 specification.
pub(crate) fn parse_app_info(input: &[u8]) -> IResult<&[u8], AppInfo> {
    let header = parse_header;
    let apps = parse_app_sections;
    let table = parse_string_table;

    let (input, (header, apps, table)) = tuple((header, apps, table))(input)?;

    Ok((input, AppInfo {
        header,
        apps,
        table,
    }))
}

/// Parse the version, magic bytes, universe and table offset from the file header.
fn parse_header(input: &[u8]) -> IResult<&[u8], AppInfoHeader> {
    let version = tag(HEADER_VERSION);
    let magic = tag(HEADER_MAGIC);
    let universe = le_u32;
    let offset = le_i64;

    let (input, (_, _, universe, offset)) = tuple((version, magic, universe, offset))(input)?;

    Ok((input, AppInfoHeader { universe, offset }))
}

/// Parse the detailed app information contained within an appinfo file.
fn parse_app_sections(input: &[u8]) -> IResult<&[u8], Vec<AppSection>> {
    let mut sections = many_till(parse_app_section, tag(b"\0\0\0\0"));
    let (input, (sections, _)) = sections(input)?;

    Ok((input, sections))
}

/// Parse a single app section.
fn parse_app_section(input: &[u8]) -> IResult<&[u8], AppSection> {
    let mut info = tuple((le_u32, le_u32, le_u32, le_u32, le_u64));
    let (input, (appid, size, info_state, last_updated, pics_token)) = info(input)?;
    let (input, sha1) = take(20usize)(input)?;
    let (input, change_number) = le_u32(input)?;
    let (input, blob) = take(size - 40)(input)?;

    Ok((input, AppSection {
        appid,
        info_state,
        last_updated,
        pics_token,
        change_number,
        blob: blob.into(),
    }))
}

/// Parse the table of null-terminated strings at the end of the appinfo file.
fn parse_string_table(input: &[u8]) -> IResult<&[u8], Vec<&str>> {
    let (input, string_count) = le_u32(input)?;
    let (input, string_table) = count(parse_nullstring, string_count as usize)(input)?;

    Ok((input, string_table))
}

/// Parse a null-terminated variable length string.
fn parse_nullstring(input: &[u8]) -> IResult<&[u8], &str> {
    let null_str = terminated(take_until("\0"), tag("\0"));
    map_res(null_str,  |s|std::str::from_utf8(s))(input)
}

