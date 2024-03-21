//! Implements helper traits for _[normalized names](NormalizedName)_.

use std::fmt::Display;

use tree_sitter::Node;

use crate::{
    prelude::{HasRawValue, NormalizedName, TypeDefinition},
    utils::get_spaces,
};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        if node.kind() == "typedName" {
            NormalizedName {
                spaces_before,
                name: node
                    .child(0)
                    .unwrap()
                    .utf8_text(code_bytes)
                    .unwrap()
                    .to_string(),
                r#type: node
                    .child(2)
                    .map(|type_node| TypeDefinition::from((type_node, code_bytes, false))),
                is_type_optional: false, //TODO
                spaces_after,
            }
        } else {
            NormalizedName {
                spaces_before,
                name: node
                    .utf8_text(code_bytes)
                    .unwrap()
                    .to_string(),
                r#type: None,
                is_type_optional: true,
                spaces_after,
            }
        }
    }
}

impl Display for NormalizedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for NormalizedName {
    fn get_raw_value(&self) -> String {
        if let Some(r#type) = &self.r#type {
            format!("{}: {}", self.name, r#type)
        } else {
            self.name.to_string()
        }
    }
}
