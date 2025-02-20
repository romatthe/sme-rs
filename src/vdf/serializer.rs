use std::ffi::CString;
use crate::vdf::parser::{VdfNode, VdfString};

pub struct VdfSerializer<'a> {
    string_table: &'a Vec<CString>,
}

impl<'a> VdfSerializer<'a> {
    pub fn new(string_table: &'a Vec<CString>) -> Self {
        VdfSerializer {
            string_table,
        }
    }

    pub fn serialize_vdf(&self, vdf: &[VdfNode]) -> anyhow::Result<String> {
        let mut buffer = String::new();
        self.serialize_vdf_nodes(&mut buffer, vdf, 0)?;

        Ok(buffer)
    }

    fn serialize_vdf_nodes(&self, buffer: &mut String, nodes: &[VdfNode], indentation: usize) -> anyhow::Result<()> {
        for node in nodes {
            self.serialize_vdf_node(buffer, &node, indentation)?;
        }

        Ok(())
    }

    fn serialize_vdf_node(&self, buffer: &mut String, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        match node {
            VdfNode::Nested { .. } => {
                self.serialize_vdf_node_nested(buffer, node, indentation)?;
            },
            VdfNode::String { .. } => {
                self.serialize_vdf_node_string(buffer, node, indentation)?;
            },
            VdfNode::Int { .. }    => {
                self.serialize_vdf_node_int(buffer, node, indentation)?;
            },
        }

        Ok(())
    }

    fn serialize_vdf_node_nested(&self, buffer: &mut String, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Nested { key, nodes} = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string(buffer, key)?;
            buffer.push_str(&format!("\n"));
            buffer.push_str(&format!("{tabs}{{\n"));
            self.serialize_vdf_nodes(buffer, nodes, indentation + 1)?;
            buffer.push_str(&format!("{tabs}}}\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_string(&self, buffer: &mut String, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::String { key, value } = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string(buffer, key)?;
            buffer.push_str(&format!("\t\t"));
            self.serialize_vdf_string(buffer, value)?;
            buffer.push_str(&format!("\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_int(&self, buffer: &mut String, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Int { key, value } = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string(buffer, key)?;
            buffer.push_str(&format!("\t\t"));
            buffer.push_str(&format!("\"{}\"\n", value));
        }

        Ok(())
    }

    fn serialize_vdf_string(&self, buffer: &mut String, string: &VdfString) -> anyhow::Result<()> {
        match string {
            VdfString::StringRef(ref_id) => {
                let ref_val = &self.string_table[*ref_id as usize];
                buffer.push_str(&format!("\"{}\"", ref_val.to_str()?));
            },
            VdfString::String(string) => {
                let string = sanitize_string(string.to_str()?);
                buffer.push_str(&format!("\"{}\"", &string));         // TODO: Does this even work properly?
            },
        };

        Ok(())
    }
}

fn sanitize_string(string: &str) -> String {
    string.replace("\\", "\\\\")
}

