//! Implements helper traits for _[normalized names](NormalizedName)_.

use std::{fmt::Display, sync::Arc};

use tree_sitter::Node;

use crate::{
    prelude::{HasRawValue, NormalizedName, Print, SingleToken, TypeDefinition},
    utils::get_spaces,
};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        if let Some(type_node) = node.child(2) {
            NormalizedName {
                spaces_before,
                name: node
                    .child(0)
                    .unwrap()
                    .utf8_text(code_bytes)
                    .unwrap()
                    .to_string(),
                colon: Some(SingleToken::from((node.child(1).unwrap(), code_bytes))),
                r#type: Some(Arc::new(TypeDefinition::from((
                    type_node, code_bytes, false,
                )))),
                spaces_after,
            }
        } else {
            NormalizedName {
                spaces_before,
                name: node.utf8_text(code_bytes).unwrap().to_string(),
                colon: None,
                r#type: None,
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

impl Print for NormalizedName {
    fn print(&self) -> String {
        format!(
            "{}{}{}",
            self.spaces_before,
            self.get_raw_value(),
            self.spaces_after
        )
    }
    fn print_leading(&self) -> String {
        format!("{}{}", self.spaces_before, self.get_raw_value())
    }
    fn print_trailing(&self) -> String {
        format!("{}{}", self.get_raw_value(), self.spaces_after)
    }
}
