//! Implements helpful traits for do block

use std::sync::Arc;

use crate::prelude::{parse_block, Ast, AstNode, DoBlock, SingleToken};

impl AstNode for DoBlock {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "doBlock" {
            return None;
        }

        Some(DoBlock {
            do_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            body: Ast {
                uri: None,
                tokens: Arc::new(parse_block(
                    node.child(1).unwrap(),
                    &mut Vec::new(),
                    code_bytes,
                )),
            },
            end_keyword: SingleToken::from((node.child(2).unwrap(), code_bytes)),
        })
    }
}
