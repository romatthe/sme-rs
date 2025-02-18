use crate::vdf::parser::{VdfNode, VdfString};

pub fn stringify_vdf(vdf: &[VdfNode]) -> anyhow::Result<String> {
    let mut buffer = String::new();
    buffer.push_str(&stringify_vdf_nodes(vdf, 0)?);

    Ok(buffer)
}

fn stringify_vdf_nodes(nodes: &[VdfNode], indentation: usize) -> anyhow::Result<String> {
    let mut temp = String::new();

    for node in nodes {
        temp.push_str(&stringify_vdf_node(&node, indentation)?);
    }

    Ok(temp)
}

fn stringify_vdf_node(node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
    match node {
        VdfNode::Nested { .. } => stringify_vdf_node_nested(node, indentation),
        VdfNode::String { .. } => stringify_vdf_node_string(node, indentation),
        VdfNode::Int { .. }    => stringify_vdf_node_int(node, indentation),
    }
}

fn stringify_vdf_node_nested(node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
    let mut temp = String::new();
    let tabs = "\t".repeat(indentation);
    if let VdfNode::Nested { key, nodes} = node {
        temp.push_str(&format!("{tabs}{}\n", stringify_vdf_string(key)?));
        temp.push_str(&format!("{tabs}{{\n"));
        temp.push_str(&format!("{}", stringify_vdf_nodes(nodes, indentation + 1)?));
        temp.push_str(&format!("{tabs}}}\n"));
    }

    Ok(temp)
}

fn stringify_vdf_node_string(node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
    let mut temp = String::new();
    let tabs = "\t".repeat(indentation);
    if let VdfNode::String { key, value } = node {
        temp.push_str(&format!("{tabs}{}\t{}\n", stringify_vdf_string(key)?, stringify_vdf_string(value)?));
    }

    Ok(temp)
}

fn stringify_vdf_node_int(node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
    let mut temp = String::new();
    let tabs = "\t".repeat(indentation);
    if let VdfNode::Int { key, value } = node {
        temp.push_str(&format!("{tabs}{}\t{}\n", stringify_vdf_string(key)?, value));
    }

    Ok("".to_string())
}

fn stringify_vdf_string(string: &VdfString) -> anyhow::Result<String> {
    let result = match string {
        VdfString::StringRef(ref_id) => {
            format!("\"{}\"", ref_id)                   // TODO: This should be looked up in the string table!!!!
        },
        VdfString::String(string) => {
            format!("\"{}\"", string.to_str()?)         // TODO: Does this even work properly?
        },
    };

    Ok(result)
}