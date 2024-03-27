//! Implements a helper `From<>` trait for `SingleToken`.

use tree_sitter::Node;

use crate::{
    prelude::SingleToken,
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
