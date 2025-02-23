use crate::v29::{AppInfo, AppInfoHeader, AppSection, HEADER_MAGIC, HEADER_VERSION};
use crate::vdf;

use indexmap::IndexMap;
use nom::AsBytes;
use sha1::{Digest, Sha1};

use std::ffi::CString;
use std::io::Write;
use std::mem::size_of_val;
use crate::vdf::serializer::VdfSerializer;

pub fn pack_app_info<S: Write>(writer: &mut S, app_info: &AppInfo) -> anyhow::Result<()> {
    // Write the entire apps section to a buffer first so we can figure out the string table offset
    let mut app_buffer = Vec::new();
    pack_app_info_apps(&mut app_buffer, &app_info.apps, app_info.table.as_slice())?;

    // Calculate the header offset value first before writing it
    let offset = app_buffer.len() + size_of_val(&[0x00, 0x00, 0x00, 0x00]);
    pack_app_info_header(writer, &app_info.header, offset as i64)?;
    writer.write(&app_buffer)?;
    pack_app_info_string_table(writer, &app_info.table)?;

    Ok(())
}

fn pack_app_info_header<S: Write>(mut writer: &mut S, header: &AppInfoHeader, offset: i64) -> anyhow::Result<()> {
    writer.write(HEADER_VERSION)?;
    writer.write(HEADER_MAGIC)?;
    writer.write(&header.universe.to_le_bytes())?;
    writer.write(&offset.to_le_bytes())?;

    Ok(())
}

fn pack_app_info_apps<S: Write>(writer: &mut S, apps: &IndexMap<u32, AppSection>, string_table: &[String]) -> anyhow::Result<()> {
    for (key, app) in apps {
        pack_app_info_app(writer, app, string_table)?;
    }

    // Mark the end of the apps section
    writer.write(&[0x00, 0x00, 0x00, 0x00])?;

    Ok(())
}

fn pack_app_info_string_table<S: Write>(mut writer: &mut S, table: &Vec<String>) -> anyhow::Result<()> {
    writer.write(&(table.len() as u32).to_le_bytes())?;  // Write the string table length

    for cs in table {
        writer.write(cs.as_bytes())?;
        writer.write(&[0])?;
    }

    Ok(())
}

fn pack_app_info_app<S: Write>(writer: &mut S, section: &AppSection, string_table: &[String]) -> anyhow::Result<()> {
    // Calculate the SHA1 of the binary VDF blob
    let mut vdf_buffer = Vec::new();
    let mut hasher = Sha1::new();
    vdf::packer::pack_vdf(&mut vdf_buffer, &section.vdf)?;
    hasher.update(&vdf_buffer);
    let sha1_binary = hasher.finalize();

    // Calculate the SHA1 of the textual VDF representation
    let mut serliazer = VdfSerializer::new(string_table);
    let serialized = serliazer.serialize_vdf(&section.vdf)?;
    let mut hasher = Sha1::new();
    hasher.update(serialized.as_bytes());
    let sha1_text = hasher.finalize();

    writer.write(&section.appid.to_le_bytes())?;
    writer.write(&(vdf_buffer.len() as u32 + 60u32).to_le_bytes())?;
    writer.write(&section.info_state.to_le_bytes())?;
    writer.write(&section.last_updated.to_le_bytes())?;
    writer.write(&section.pics_token.to_le_bytes())?;
    // writer.write(&section.sha1_text)?;                      // TODO: Use the actual SHA1 value for the text here...
    writer.write(&sha1_text)?;
    writer.write(&section.change_number.to_le_bytes())?;
    writer.write(sha1_binary.as_bytes())?;
    writer.write(&vdf_buffer)?;

    Ok(())
}


