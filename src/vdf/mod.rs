pub(crate) mod parser;
pub(crate) mod packer;
pub(crate) mod serializer;

#[derive(Clone, Debug)]
pub struct VdfNode {
    pub(crate) key: VdfStringRef,
    pub(crate) value: VdfNodeKind,
}

#[derive(Clone, Debug)]
pub enum VdfNodeKind {
    Nested {
        /// This cannot be made info a hashmap, since some apps have duplicate key/values. Perhaps
        /// it's fine to remove these duplicate values, but I'm erring on the side of caution and just
        /// leaving the duplicate key/values in, as it makes it easier to validate the packing logic.
        nodes: Vec<VdfNode>,
    },
    String {
        value: String,
    },
    Int {
        value: u32,
    },
}

#[derive(Clone, Debug)]
pub struct VdfStringRef {
    pub string_ref: u32,
    pub string: Option<String>,
}

impl VdfNode {
    pub fn complete_string_refs(&mut self, string_table: &[String]) {
        // Add the concrete string value from string table to the ref
        self.key.string = Some(string_table[self.key.string_ref as usize].clone());

        match self.value {
            VdfNodeKind::Nested { ref mut nodes } => {
                for mut node in nodes {
                    node.complete_string_refs(string_table);
                }
            },
            VdfNodeKind::String { .. } => { },
            VdfNodeKind::Int { .. } => { },
        }
    }

    pub fn get(&self, index: &str) -> Option<&Self> {
        match &self.value {
            VdfNodeKind::Nested { nodes } => {
                for node in nodes {
                    if node.key.string == Some(index.to_string()) {
                        return Some(node)
                    }
                }
                None
            },
            VdfNodeKind::String { .. } => None,
            VdfNodeKind::Int { .. } => None,
        }
    }

    pub fn get_mut(&mut self, index: &str) -> Option<&mut Self> {
        match &mut self.value {
            VdfNodeKind::Nested { nodes } => {
                for node in nodes {
                    if node.key.string == Some(index.to_string()) {
                        return Some(node)
                    }
                }
                None
            },
            VdfNodeKind::String { .. } => None,
            VdfNodeKind::Int { .. } => None,
        }
    }
}