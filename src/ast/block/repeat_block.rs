//! Implements helper traits for repeat blocks.

use std::sync::Arc;

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        parse_block, Expression, FromNode, HasRange, LuauStatement, Range, RepeatBlock, Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for RepeatBlock {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "repeatBlock" {
            return None;
        }

        Some(RepeatBlock {
            repeat_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            until_keyword: Token::from_node(node.child_by_field_name("until")?, code_bytes)?,
            condition: Expression::from_node(node.child_by_field_name("condition")?, code_bytes)
                .map(Arc::new)?,
        })
    }
}

impl HasRange for RepeatBlock {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.repeat_keyword.get_range(), self.condition.get_range())
    }
}
