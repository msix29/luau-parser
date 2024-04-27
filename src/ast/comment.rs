//! Implements helper traits for comments.

use crate::prelude::{Comment, HasRange, LuauStatement, Range, Token};

impl LuauStatement for Comment {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "comment" {
            return None;
        }

        Some(Self(Token::from((node, code_bytes))))
    }
}

impl HasRange for Comment {
    fn get_range(&self) -> Range {
        self.0.get_range()
    }
}
