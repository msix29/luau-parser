//! Implements helper traits for while loops.

use std::sync::Arc;

use crate::{
    prelude::{DoBlock, Expression, FromNode, HasRange, LuauStatement, Range, Token, WhileLoop},
    utils::get_range_from_boundaries,
};

impl LuauStatement for WhileLoop {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "whileLoop" {
            return None;
        }

        Some(WhileLoop {
            while_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            condition: Expression::from_node(node.child(1)?, code_bytes).map(Arc::new)?,
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasRange for WhileLoop {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.while_keyword.get_range(), self.do_block.get_range())
    }
}
