//! Implements helper traits for while loops.

use crate::prelude::{AstNode, DoBlock, Expression, SingleToken, WhileLoop};

impl AstNode for WhileLoop {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "whileLoop" {
            return None;
        }

        Some(WhileLoop {
            while_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            condition: Expression::from((node.child(1).unwrap(), code_bytes)),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}
