//! Implements helper traits for numerical for loops.

use std::sync::Arc;

use crate::{
    prelude::{
        DoBlock, Expression, HasRange, LuauStatement, NormalizedName, NumericalFor, Range,
        Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for NumericalFor {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "numericalFor" {
            return None;
        }

        Some(NumericalFor {
            for_keyword: Token::from((node.child(0).unwrap(), code_bytes)),
            variable: NormalizedName::from((node.child(1).unwrap(), code_bytes)),
            equal_keyword: Token::from((node.child(2).unwrap(), code_bytes)),
            start: Arc::new(Expression::from((node.child(3).unwrap(), code_bytes))),
            start_comma: Token::from((node.child(4).unwrap(), code_bytes)),
            end: Arc::new(Expression::from((node.child(5).unwrap(), code_bytes))),
            end_comma: node
                .child(6)
                .map(|node| Token::from((node, code_bytes))),
            step: node
                .child(7)
                .map(|node| Arc::new(Expression::from((node, code_bytes)))),
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
