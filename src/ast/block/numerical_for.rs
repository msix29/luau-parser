//! Implements helper traits for numerical for loops.

use crate::prelude::{AstNode, DoBlock, Expression, NormalizedName, NumericalFor, SingleToken};

impl AstNode for NumericalFor {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "numericalFor" {
            return None;
        }

        Some(NumericalFor {
            for_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            variable: NormalizedName::from((node.child(1).unwrap(), code_bytes)),
            equal_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            start: Expression::from((node.child(3).unwrap(), code_bytes)),
            start_comma: SingleToken::from((node.child(4).unwrap(), code_bytes)),
            end: Expression::from((node.child(5).unwrap(), code_bytes)),
            end_comma: node
                .child(6)
                .map(|node| SingleToken::from((node, code_bytes))),
            step: node
                .child(7)
                .map(|node| Expression::from((node, code_bytes))),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}
