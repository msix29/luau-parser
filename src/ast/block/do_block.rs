//! Implements helper traits for do block

use std::sync::Arc;

use crate::{
    prelude::{parse_block, Ast, AstNode, DoBlock, HasLocation, SingleToken},
    utils::get_location_from_boundaries,
};

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

impl HasLocation for DoBlock {
    fn get_location(&self) -> crate::prelude::Location {
        get_location_from_boundaries(
            self.do_keyword.get_location(),
            self.end_keyword.get_location(),
        )
    }
}
