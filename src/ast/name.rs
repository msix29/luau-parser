//! Implements helper traits for _[normalized names](NormalizedName)_.

use tree_sitter::Node;

use crate::prelude::{TypeDefinition, NormalizedName};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        if node.kind() == "typedName" {
            NormalizedName {
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
            }
        } else {
            //TODO: `name` by itself.
            NormalizedName::default()
        }
    }
}
