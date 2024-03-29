//! Implements helper traits for repeat blocks.

use std::sync::Arc;

use crate::prelude::{parse_block, Ast, AstNode, Expression, RepeatBlock, SingleToken};

impl AstNode for RepeatBlock {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "repeatBlock" {
            return None;
        }

        Some(RepeatBlock {
            repeat_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            body: node
                .child_by_field_name("body")
                .map(|body| Ast {
                    uri: None,
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                })
                .unwrap_or_default(),
            until_keyword: SingleToken::from((
                node.child_by_field_name("until").unwrap(),
                code_bytes,
            )),
            condition: Expression::from((
                node.child_by_field_name("condition").unwrap(),
                code_bytes,
            )),
        })
    }
}
