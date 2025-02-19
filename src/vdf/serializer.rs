use std::ffi::CString;
use crate::vdf::parser::{VdfNode, VdfString};

pub struct Serializer<'a> {
    string_table: &'a Vec<CString>,
    buffer: String,
}

impl<'a> Serializer<'a> {
    pub fn new(string_table: &'a Vec<CString>) -> Self {
        Serializer {
            string_table,
            buffer: String::new(),
        }
    }

    pub fn serialize_vdf(mut self, vdf: &[VdfNode]) -> anyhow::Result<String> {
        &mut self.serialize_vdf_nodes(vdf, 0)?;

        Ok(self.buffer)
    }

    fn serialize_vdf_nodes(&mut self, nodes: &[VdfNode], indentation: usize) -> anyhow::Result<()> {
        for node in nodes {
            &mut self.serialize_vdf_node(&node, indentation)?;
        }

        Ok(())
    }

    fn serialize_vdf_node(&mut self, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        match node {
            VdfNode::Nested { .. } => {
                &mut self.serialize_vdf_node_nested(node, indentation)?;
            },
            VdfNode::String { .. } => {
                &mut self.serialize_vdf_node_string(node, indentation)?;
            },
            VdfNode::Int { .. }    => {
                &mut self.serialize_vdf_node_int(node, indentation)?;
            },
        }

        Ok(())
    }

    fn serialize_vdf_node_nested(&mut self, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Nested { key, nodes} = node {
            &self.buffer.push_str(&format!("{tabs}"));
            &self.serialize_vdf_string(key)?;
            &self.buffer.push_str(&format!("\n"));
            &self.buffer.push_str(&format!("{tabs}{{\n"));
            &self.serialize_vdf_nodes(nodes, indentation + 1)?;
            &self.buffer.push_str(&format!("{tabs}}}\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_string(&mut self, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::String { key, value } = node {
            &self.buffer.push_str(&format!("{tabs}"));
            &self.serialize_vdf_string(key)?;
            &self.buffer.push_str(&format!("\t"));
            &self.serialize_vdf_string(value)?;
            &self.buffer.push_str(&format!("\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_int(&mut self, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Int { key, value } = node {
            &self.buffer.push_str(&format!("{tabs}"));
            &self.serialize_vdf_string(key)?;
            &self.buffer.push_str(&format!("\t"));
            &self.buffer.push_str(&format!("{}\n", value));
        }

        Ok(())
    }

    fn serialize_vdf_string(&mut self, string: &VdfString) -> anyhow::Result<()> {
        let result = match string {
            VdfString::StringRef(ref_id) => {
                &self.buffer.push_str(&format!("\"{}\"", ref_id));                   // TODO: This should be looked up in the string table!!!!
            },
            VdfString::String(string) => {
                &self.buffer.push_str(&format!("\"{}\"", string.to_str()?));         // TODO: Does this even work properly?
            },
        };

        Ok(())
    }
}