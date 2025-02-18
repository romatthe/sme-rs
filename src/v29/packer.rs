use std::ffi::CString;
use crate::v29::{AppInfo, AppInfoHeader, AppSection, HEADER_MAGIC, HEADER_VERSION};
use std::io::Write;
use nom::AsBytes;
use sha1::{Digest, Sha1};
use crate::vdf;

pub fn pack_app_info<S: Write>(writer: &mut S, app_info: &AppInfo) -> anyhow::Result<()> {
    pack_app_info_header(writer, &app_info.header)?;
    pack_app_info_apps(writer, &app_info.apps)?;
    pack_app_info_string_table(writer, &app_info.table)?;

    Ok(())
}

fn pack_app_info_header<S: Write>(mut writer: &mut S, header: &AppInfoHeader) -> anyhow::Result<()> {
    writer.write(HEADER_VERSION)?;
    writer.write(HEADER_MAGIC)?;
    writer.write(&header.universe.to_le_bytes())?;
    writer.write(&header.offset.to_le_bytes())?;

    Ok(())
}

fn pack_app_info_apps<S: Write>(writer: &mut S, apps: &[AppSection]) -> anyhow::Result<()> {
    for app in apps {
        pack_app_info_app(writer, &app)?;
    }

    // Mark the end of the apps section
    writer.write(&[0x00, 0x00, 0x00, 0x00])?;

    Ok(())
}

fn pack_app_info_string_table<S: Write>(mut writer: &mut S, table: &Vec<CString>) -> anyhow::Result<()> {
    writer.write(&(table.len() as u32).to_le_bytes())?;  // Write the string table length

    for cs in table {
        writer.write(cs.as_bytes())?;
        writer.write(&[0])?;
    }

    Ok(())
}

fn pack_app_info_app<S: Write>(writer: &mut S, section: &AppSection) -> anyhow::Result<()> {
    let mut vdf_buffer = Vec::new();
    vdf::packer::pack_vdf(&mut vdf_buffer, section.vdf.as_slice())?;

    let mut hasher = Sha1::new();
    hasher.update(&vdf_buffer);
    let sha1_binary = hasher.finalize();

    writer.write(&section.appid.to_le_bytes())?;
    writer.write(&(vdf_buffer.len() as u32 + 60u32).to_le_bytes())?;
    writer.write(&section.info_state.to_le_bytes())?;
    writer.write(&section.last_updated.to_le_bytes())?;
    writer.write(&section.pics_token.to_le_bytes())?;
    writer.write(&section.sha1_text)?;
    writer.write(&section.change_number.to_le_bytes())?;
    writer.write(sha1_binary.as_bytes())?;
    writer.write(&vdf_buffer)?;

    Ok(())
}


