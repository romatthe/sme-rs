use std::ffi::CString;
use indexmap::IndexMap;
use crate::vdf::parser::{VdfNode, VdfStringRef};

pub struct VdfSerializer<'a> {
    string_table: &'a Vec<CString>,
}

impl<'a> VdfSerializer<'a> {
    pub fn new(string_table: &'a Vec<CString>) -> Self {
        VdfSerializer {
            string_table,
        }
    }

    pub fn serialize_vdf(&self, vdf: &IndexMap<VdfStringRef, VdfNode>) -> anyhow::Result<String> {
        let mut buffer = String::new();
        self.serialize_vdf_nodes(&mut buffer, vdf, 0)?;

        Ok(buffer)
    }

    fn serialize_vdf_nodes(&self, buffer: &mut String, nodes: &IndexMap<VdfStringRef, VdfNode>, indentation: usize) -> anyhow::Result<()> {
        for (key, node) in nodes {
            self.serialize_vdf_node(buffer, key, node, indentation)?;
        }

        Ok(())
    }

    fn serialize_vdf_node(&self, buffer: &mut String, key: &VdfStringRef, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        match node {
            VdfNode::Nested { .. } => {
                self.serialize_vdf_node_nested(buffer, key, node, indentation)?;
            },
            VdfNode::String { .. } => {
                self.serialize_vdf_node_string(buffer, key, node, indentation)?;
            },
            VdfNode::Int { .. }    => {
                self.serialize_vdf_node_int(buffer, key, node, indentation)?;
            },
        }

        Ok(())
    }

    fn serialize_vdf_node_nested(&self, buffer: &mut String, key: &VdfStringRef, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Nested { nodes} = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string_ref(buffer, key)?;
            buffer.push_str(&format!("\n"));
            buffer.push_str(&format!("{tabs}{{\n"));
            self.serialize_vdf_nodes(buffer, nodes, indentation + 1)?;
            buffer.push_str(&format!("{tabs}}}\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_string(&self, buffer: &mut String, key: &VdfStringRef, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::String { value } = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string_ref(buffer, key)?;
            buffer.push_str(&format!("\t\t"));
            self.serialize_vdf_string(buffer, value)?;
            buffer.push_str(&format!("\n"));
        }

        Ok(())
    }

    fn serialize_vdf_node_int(&self, buffer: &mut String, key: &VdfStringRef, node: &VdfNode, indentation: usize) -> anyhow::Result<()> {
        let tabs = "\t".repeat(indentation);
        if let VdfNode::Int { value } = node {
            buffer.push_str(&format!("{tabs}"));
            self.serialize_vdf_string_ref(buffer, key)?;
            buffer.push_str(&format!("\t\t"));
            buffer.push_str(&format!("\"{}\"\n", value));
        }

        Ok(())
    }

    fn serialize_vdf_string(&self, buffer: &mut String, string: &String) -> anyhow::Result<()> {
        let string = sanitize_string(string);
        buffer.push_str(&format!("\"{}\"", &string));         // TODO: Does this even work properly?

        Ok(())
    }

    fn serialize_vdf_string_ref(&self, buffer: &mut String, ref_id: &VdfStringRef) -> anyhow::Result<()> {
        if let VdfStringRef(id) = ref_id {
            let ref_val = &self.string_table[*id as usize];
            buffer.push_str(&format!("\"{}\"", ref_val.to_str()?));
        }

        Ok(())
    }
}

fn sanitize_string(string: &str) -> String {
    string.replace("\\", "\\\\")
}

