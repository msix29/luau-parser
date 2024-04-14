//! Implements helper traits for comments.

use crate::prelude::{Comment, HasRange, Range, LuauStatement, SingleToken};

impl LuauStatement for Comment {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "comment" {
            return None;
        }

        Some(Comment(SingleToken::from((node, code_bytes))))
    }
}

impl HasRange for Comment {
    fn get_range(&self) -> Range {
        self.0.get_range()
    }
}
