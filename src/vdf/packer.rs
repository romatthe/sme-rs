use std::io::Write;
use crate::vdf::{VdfNode, VdfString, VdfStringRef};

pub fn pack_vdf<S: Write>(writer: &mut S, vdf: &[(VdfStringRef, VdfNode)]) -> anyhow::Result<()> {
    pack_vdf_nodes(writer, vdf)?;

    Ok(())
}

fn pack_vdf_nodes<S: Write>(writer: &mut S, nodes: &[(VdfStringRef, VdfNode)]) -> anyhow::Result<()> {
    for node in nodes {
        pack_vdf_node(writer, node)?;
    }
    writer.write(&[0x08])?;

    Ok(())
}

fn pack_vdf_node<S: Write>(writer: &mut S, node: &(VdfStringRef, VdfNode)) -> anyhow::Result<()> {
    match node {
        (_, VdfNode::Nested { .. }) => pack_vdf_node_nested(writer, node)?,
        (_, VdfNode::String { .. }) => pack_vdf_node_string(writer, node)?,
        (_, VdfNode::Int { .. })    => pack_vdf_node_int(writer, node)?,
    }

    Ok(())
}

fn pack_vdf_node_nested<S: Write>(writer: &mut S, node: &(VdfStringRef, VdfNode)) -> anyhow::Result<()> {
    if let (key, VdfNode::Nested { nodes}) = node {
        writer.write(&[0x00])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;      // Key
        pack_vdf_nodes(writer, nodes)?;     // Value
    }

    Ok(())
}

fn pack_vdf_node_string<S: Write>(writer: &mut S, node: &(VdfStringRef, VdfNode)) -> anyhow::Result<()> {
    if let (key, VdfNode::String { value }) = node {
        writer.write(&[0x01])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;      // Key
        pack_vdf_string(writer, value)?;    // Value
    }

    Ok(())
}

fn pack_vdf_node_int<S: Write>(writer: &mut S, node: &(VdfStringRef, VdfNode)) -> anyhow::Result<()> {
    if let (key, VdfNode::Int { value }) = node {
        writer.write(&[0x02])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;      // Key
        writer.write(&value.to_le_bytes())?;// Value
    }

    Ok(())
}

fn pack_vdf_string<S: Write>(writer: &mut S, string: &VdfString) -> anyhow::Result<()> {
    match string {
        VdfString::StringRef(ref_id) => {
            writer.write(&ref_id.to_le_bytes())?;
        },
        VdfString::String(string) => {
            writer.write(string.as_bytes_with_nul())?;
        },
    }

    Ok(())
}

fn pack_vdf_string_ref<S: Write>(writer: &mut S, string_ref: &VdfStringRef) -> anyhow::Result<()> {
    let VdfStringRef(ref_id) = string_ref;
    writer.write(&ref_id.to_le_bytes())?;

    Ok(())
}
