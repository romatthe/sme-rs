use std::io::Write;
use crate::vdf::{VdfNode, VdfNodeKind, VdfStringRef};

pub fn pack_vdf<S: Write>(writer: &mut S, vdf: &[VdfNode]) -> anyhow::Result<()> {
    pack_vdf_nodes(writer, vdf)?;

    Ok(())
}

fn pack_vdf_nodes<S: Write>(writer: &mut S, nodes: &[VdfNode]) -> anyhow::Result<()> {
    for node in nodes {
        pack_vdf_node(writer, node)?;
    }
    writer.write(&[0x08])?;

    Ok(())
}

fn pack_vdf_node<S: Write>(writer: &mut S, node: &VdfNode) -> anyhow::Result<()> {
    match node {
        VdfNode { key, value: VdfNodeKind::Nested { .. } } => {
            pack_vdf_node_nested(writer, node)?
        },
        VdfNode { key, value: VdfNodeKind::String { .. } } => {
            pack_vdf_node_string(writer, node)?
        },
        VdfNode { key, value: VdfNodeKind::Int { .. } } => {
            pack_vdf_node_int(writer, node)?
        },
    }

    Ok(())
}

fn pack_vdf_node_nested<S: Write>(writer: &mut S, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode { key, value: VdfNodeKind::Nested { nodes } } = node {
        writer.write(&[0x00])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;  // Key
        pack_vdf_nodes(writer, nodes)?;     // Value
    }

    Ok(())
}

fn pack_vdf_node_string<S: Write>(writer: &mut S, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode { key, value: VdfNodeKind::String { value } } = node {
        writer.write(&[0x01])?;         // Magic byte
        pack_vdf_string_ref(writer, key)?;  // Key
        pack_vdf_string(writer, value)?;    // Value
    }

    Ok(())
}

fn pack_vdf_node_int<S: Write>(writer: &mut S, node: &VdfNode) -> anyhow::Result<()> {
    if let VdfNode { key, value: VdfNodeKind::Int { value } } = node {
        writer.write(&[0x02])?;          // Magic byte
        pack_vdf_string_ref(writer, key)?;   // Key
        writer.write(&value.to_le_bytes())?; // Value
    }

    Ok(())
}

fn pack_vdf_string<S: Write>(writer: &mut S, string: &str) -> anyhow::Result<()> {
    writer.write(string.as_bytes())?;
    writer.write(&[0])?;

    Ok(())
}

fn pack_vdf_string_ref<S: Write>(writer: &mut S, string_ref: &VdfStringRef) -> anyhow::Result<()> {
    let VdfStringRef { string_ref, .. } = string_ref;
    writer.write(&string_ref.to_le_bytes())?;

    Ok(())
}
