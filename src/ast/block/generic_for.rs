//! Implemens helper traits for for-in loops.

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        DoBlock, Expression, FromNode, GenericFor, HasRange, List, LuauStatement, NormalizedName,
        Range, Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for GenericFor {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "forIn" {
            return None;
        }

        Some(GenericFor {
            for_keyword: Token::from_node(node.child_by_field_name("for")?, code_bytes)?,
            names: List::from_iter(
                node.children_by_field_name("binding", cursor),
                node,
                "separator",
                code_bytes,
                |_, binding| NormalizedName::from_node(binding.child(0)?, code_bytes),
            ),
            in_keyword: Token::from_node(node.child_by_field_name("in")?, code_bytes)?,
            expressions: Expression::from_nodes(
                node.children_by_field_name("value", cursor),
                code_bytes,
            ),
            do_block: DoBlock::try_from_node(
                node.child_by_field_name("doBlock").unwrap(),
                cursor,
                code_bytes,
            )
            .unwrap(),
        })
    }
}

impl HasRange for GenericFor {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.for_keyword.get_range(), self.do_block.get_range())
    }
}
