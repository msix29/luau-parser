//! Implements helper trait for [`local assignments`](LocalAssignment).

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        Expression, FromNode, HasRange, List, LocalAssignment, LuauStatement, NormalizedName,
        Range, Token,
    },
    utils::{get_range_from_boundaries, map_option},
};

impl LuauStatement for LocalAssignment {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "variableDeclaration" {
            return None;
        }

        Some(Self {
            local_token: Token::from_node(node.child(0)?, code_bytes)?,
            name_list: List::from_iter(
                node.children_by_field_name("binding", cursor),
                node,
                "separator",
                code_bytes,
                |_, binding| NormalizedName::from_node(binding, code_bytes),
            ),
            equal_token: map_option(node.child_by_field_name("equal"), |equal| {
                Token::from_node(equal?, code_bytes)
            }),
            expressions: Expression::from_nodes(
                node.children_by_field_name("value", cursor),
                code_bytes,
            ),
        })
    }
}

impl HasRange for LocalAssignment {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.local_token.get_range(),
            self.expressions.last().map_or_else(
                || self.name_list.last().unwrap().get_range(),
                |item| item.get_range(),
            ),
        )
    }
}
