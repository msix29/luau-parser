//! Implements helper traits for _[normalized names](NormalizedName)_.

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{NormalizedName, SingleToken, TypeDefinition},
    utils::{get_location, get_spaces},
};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        if let Some(type_node) = node.child(2) {
            NormalizedName {
                location: get_location(node),
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
                location: get_location(node),
            }
        }
    }
}
