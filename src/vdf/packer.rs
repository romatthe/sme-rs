use std::ffi::CString;
use std::io::Write;
use indexmap::IndexMap;
use nom::AsBytes;
use crate::vdf::parser::{VdfNode, VdfStringRef};

pub fn pack_vdf<S: Write>(writer: &mut S, vdf: &IndexMap<VdfStringRef, VdfNode>) -> anyhow::Result<()> {
    pack_vdf_nodes(writer, vdf)?;

    Ok(())
}

fn pack_vdf_nodes<S: Write>(writer: &mut S, nodes: &IndexMap<VdfStringRef, VdfNode>) -> anyhow::Result<()> {
    for (key, node) in nodes {
        pack_vdf_node(writer, key, node)?;
    }
    writer.write(&[0x08])?;

    Ok(())
}

fn pack_vdf_node<S: Write>(writer: &mut S, key: &VdfStringRef, node: &VdfNode) -> anyhow::Result<()> {
    match node {
        VdfNode::Nested { .. } => pack_vdf_node_nested(writer, key, node)?,
        VdfNode::String { .. } => pack_vdf_node_string(writer, key, node)?,
        VdfNode::Int { .. }    => pack_vdf_node_int(writer, key, node)?,
    }

    Ok(())
}

fn pack_vdf_node_nested<S: Write>(writer: &mut S, key: &VdfStringRef, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode::Nested { nodes} = node {
        writer.write(&[0x00])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;  // Key
        pack_vdf_nodes(writer, nodes)?;     // Value
    }

    Ok(())
}

fn pack_vdf_node_string<S: Write>(writer: &mut S, key: &VdfStringRef, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode::String { value } = node {
        writer.write(&[0x01])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;  // Key
        pack_vdf_string(writer, value)?;    // Value
    }

    Ok(())
}

fn pack_vdf_node_int<S: Write>(writer: &mut S, key: &VdfStringRef, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode::Int { value } = node {
        writer.write(&[0x02])?;             // Magic byte
        pack_vdf_string_ref(writer, key)?;      // Key
        writer.write(&value.to_le_bytes())?;    // Value
    }

    Ok(())
}

fn pack_vdf_string<S: Write>(writer: &mut S, string: &String) -> anyhow::Result<()> {
    writer.write(CString::new(string.as_bytes())?.as_bytes_with_nul())?; // Todo: Perhaps find a better way :)

    Ok(())
}

fn pack_vdf_string_ref<S: Write>(writer: &mut S, ref_id: &VdfStringRef) -> anyhow::Result<()> {
    if let VdfStringRef(id) = ref_id {
        writer.write(&(*id).to_le_bytes())?;
    }

    Ok(())
}
