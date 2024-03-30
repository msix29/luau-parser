//! Implements helper traits for _[normalized names](NormalizedName)_.

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    call_any,
    prelude::{HasLocation, Location, NormalizedName, SingleToken, TypeDefinition},
    utils::get_location_from_boundaries,
};

impl From<(Node<'_>, &[u8])> for NormalizedName {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        if let Some(type_node) = node.child(2) {
            NormalizedName {
                name: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                colon: Some(SingleToken::from((node.child(1).unwrap(), code_bytes))),
                r#type: Some(Arc::new(TypeDefinition::from((
                    type_node, code_bytes, false,
                )))),
            }
        } else {
            NormalizedName {
                name: SingleToken::from((node, code_bytes)),
                colon: None,
                r#type: None,
            }
        }
    }
}

impl HasLocation for NormalizedName {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.name.get_location(),
            call_any!(get_location, self.name, self.r#type),
        )
    }
}
