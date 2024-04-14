//! Implements helper traits for do block

use std::sync::Arc;

use crate::{
    prelude::{parse_block, Ast, DoBlock, HasRange, Range, LuauStatement, SingleToken},
    utils::get_range_from_boundaries,
};

impl LuauStatement for DoBlock {
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
            body: node
                .child_by_field_name("body")
                .map(|body| Ast {
                    uri: None,
                    tokens: Arc::new(parse_block(body, &mut Vec::new(), code_bytes)),
                })
                .unwrap_or_default(),
            end_keyword: SingleToken::from((node.child_by_field_name("end").unwrap(), code_bytes)),
        })
    }
}

impl HasRange for DoBlock {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.do_keyword.get_range(),
            self.end_keyword.get_range(),
        )
    }
}
