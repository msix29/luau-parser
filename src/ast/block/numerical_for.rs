//! Implements helper traits for numerical for loops.

use std::sync::Arc;

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        DoBlock, Expression, FromNode, HasRange, LuauStatement, NormalizedName, NumericalFor,
        Range, Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for NumericalFor {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "numericalFor" {
            return None;
        }

        Some(NumericalFor {
            for_keyword: Token::from_node(node.child(0)?, code_bytes)?,
            variable: NormalizedName::from_node(node.child(1)?, code_bytes)?,
            equal_keyword: Token::from_node(node.child(2)?, code_bytes)?,
            start: Expression::from_node(node.child(3)?, code_bytes).map(Arc::new)?,
            start_comma: Token::from_node(node.child(4)?, code_bytes)?,
            end: Expression::from_node(node.child(5)?, code_bytes).map(Arc::new)?,
            end_comma: node
                .child(6)
                .map(|node| Token::from_node(node, code_bytes).unwrap()), //todo
            step: node
                .child(7)
                .map(|node| Expression::from_node(node, code_bytes).map(Arc::new))?,
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasRange for NumericalFor {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.for_keyword.get_range(), self.do_block.get_range())
    }
}
