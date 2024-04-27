//! Implements helper traits for [`normalized names`](NormalizedName).

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    call_any,
    prelude::{HasRange, NormalizedName, Range, SingleToken, TypeDefinition, TypeValue},
    utils::get_range_from_boundaries,
};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        if let Some(type_node) = node.child(2) {
            Self {
                name: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                colon: Some(SingleToken::from((node.child(1).unwrap(), code_bytes))),
                r#type: Some(Arc::new(TypeValue::from((type_node, code_bytes)))),
            }
        } else {
            Self {
                name: SingleToken::from((node, code_bytes)),
                colon: None,
                r#type: None,
            }
        }
    }
}

impl HasRange for NormalizedName {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.name.get_range(),
            call_any!(get_range, self.name, self.r#type),
        )
    }
}
