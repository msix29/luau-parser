//! Implements helper traits for repeat blocks.

use crate::{
    prelude::{parse_block, Expression, HasRange, LuauStatement, Range, RepeatBlock, SingleToken},
    utils::get_range_from_boundaries,
};

impl LuauStatement for RepeatBlock {
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
                .map(|body| parse_block(&body, code_bytes, None))
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

impl HasRange for RepeatBlock {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.repeat_keyword.get_range(), self.condition.get_range())
    }
}
