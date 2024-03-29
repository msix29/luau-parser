//! Implements a helper `From<>` trait for `SingleToken`.

use tree_sitter::Node;

use crate::{
    prelude::{HasLocation, Location, SingleToken},
    utils::{get_location, get_spaces},
};

impl From<(Node<'_>, &[u8])> for SingleToken {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let word = node.utf8_text(code_bytes).unwrap().to_string();
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            word,
            spaces_after,
            location: get_location(node),
        }
    }
}

impl From<&str> for SingleToken {
    fn from(value: &str) -> Self {
        Self {
            spaces_before: "".to_string(),
            word: value.to_string(),
            spaces_after: "".to_string(),
            location: Location::default(),
        }
    }
}

impl HasLocation for &SingleToken {
    fn get_location(&self) -> crate::prelude::Location {
        self.location
    }
}
impl HasLocation for SingleToken {
    fn get_location(&self) -> crate::prelude::Location {
        self.location
    }
}
