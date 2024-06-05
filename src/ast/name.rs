//! Implements helper traits for [`normalized names`](NormalizedName).

use std::sync::Arc;
use tree_sitter::Node;

#[cfg(feature = "references")]
use crate::types::References;
use crate::{
    call_any,
    types::{FromNode, HasRange, NormalizedName, Range, Token, TypeValue},
    utils::get_range_from_boundaries,
};

impl FromNode for NormalizedName {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        if let Some(type_node) = node.child(2) {
            Some(Self {
                name: Token::from_node(node.child(0)?, code_bytes)?,
                colon: Token::from_node(node.child(1)?, code_bytes),
                r#type: TypeValue::from_node(type_node, code_bytes).map(Arc::new),
                #[cfg(feature = "references")]
                references: References::new(),
            })
        } else {
            Some(Self {
                name: Token::from_node(node, code_bytes)?,
                colon: None,
                r#type: None,
                #[cfg(feature = "references")]
                references: References::new(),
            })
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
