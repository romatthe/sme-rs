use std::ffi::CString;
use crate::vdf::parser::{VdfNode, VdfString};

pub struct Serializer<'a> {
    string_table: &'a Vec<CString>,
}

impl<'a> Serializer<'a> {
    pub fn new(string_table: &'a Vec<CString>) -> Self {
        Serializer {
            string_table,
        }
    }

    pub fn serialize_vdf(self, vdf: &[VdfNode]) -> anyhow::Result<String> {
        let mut buffer = String::new();
        buffer.push_str(&self.serialize_vdf_nodes(vdf, 0)?);

        Ok(buffer)
    }

    fn serialize_vdf_nodes(&self, nodes: &[VdfNode], indentation: usize) -> anyhow::Result<String> {
        let mut temp = String::new();

        for node in nodes {
            temp.push_str(&self.serialize_vdf_node(&node, indentation)?);
        }

        Ok(temp)
    }

    fn serialize_vdf_node(&self, node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
        match node {
            VdfNode::Nested { .. } => self.serialize_vdf_node_nested(node, indentation),
            VdfNode::String { .. } => self.serialize_vdf_node_string(node, indentation),
            VdfNode::Int { .. }    => self.serialize_vdf_node_int(node, indentation),
        }
    }

    fn serialize_vdf_node_nested(&self, node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
        let mut temp = String::new();
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Nested { key, nodes} = node {
            temp.push_str(&format!("{tabs}{}\n", &self.serialize_vdf_string(key)?));
            temp.push_str(&format!("{tabs}{{\n"));
            temp.push_str(&format!("{}", &self.serialize_vdf_nodes(nodes, indentation + 1)?));
            temp.push_str(&format!("{tabs}}}\n"));
        }

        Ok(temp)
    }

    fn serialize_vdf_node_string(&self, node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
        let mut temp = String::new();
        let tabs = "\t".repeat(indentation);
        if let VdfNode::String { key, value } = node {
            temp.push_str(&format!("{tabs}{}\t{}\n", &self.serialize_vdf_string(key)?, &self.serialize_vdf_string(value)?));
        }

        Ok(temp)
    }

    fn serialize_vdf_node_int(&self, node: &VdfNode, indentation: usize) -> anyhow::Result<String> {
        let mut temp = String::new();
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Int { key, value } = node {
            temp.push_str(&format!("{tabs}{}\t{}\n", &self.serialize_vdf_string(key)?, value));
        }

        Ok("".to_string())
    }

    fn serialize_vdf_string(&self, string: &VdfString) -> anyhow::Result<String> {
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
}