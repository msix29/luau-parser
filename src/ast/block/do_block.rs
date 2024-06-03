//! Implements helper traits for do block

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{parse_block, DoBlock, FromNode, HasRange, LuauStatement, Range, Token},
    utils::get_range_from_boundaries,
};

impl LuauStatement for DoBlock {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "doBlock" {
            return None;
        }

        Some(DoBlock {
            do_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            body: node
                .child_by_field_name("body")
                .map(|body| parse_block(&body, code_bytes, None))
                .unwrap_or_default(),
            end_keyword: Token::from_node(node.child_by_field_name("end")?, code_bytes)?,
        })
    }
}

impl HasRange for DoBlock {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.do_keyword.get_range(), self.end_keyword.get_range())
    }
}
