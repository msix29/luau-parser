//! Implements helper traits for set expressions.

use std::sync::Arc;

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        CompoundSetExpression, Expression, FromNode, HasRange, List, LuauStatement, PrefixExp,
        Range, SetExpression, Token,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for SetExpression {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "setExpression" {
            return None;
        }

        Some(Self {
            variables: List::from_iter(
                node.children_by_field_name("variable", &mut node.walk()),
                node,
                "separator",
                code_bytes,
                |_, node| match PrefixExp::from_node(node, code_bytes)? {
                    PrefixExp::Var(var) => Some(var),
                    _ => unreachable!(),
                },
            ),
            equal: Token::from_node(node.child_by_field_name("equal")?, code_bytes)?,
            values: Expression::from_nodes(
                node.children_by_field_name("value", &mut node.walk()),
                code_bytes,
            ),
        })
    }
}
impl HasRange for SetExpression {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.variables.last().unwrap().get_range(),
            self.values.last().unwrap().get_range(),
        )
    }
}

impl LuauStatement for CompoundSetExpression {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "compoundSetExpression" {
            return None;
        }
        let variable = match PrefixExp::from_node(node.child(0)?, code_bytes)? {
            PrefixExp::Var(var) => var,
            _ => unreachable!(),
        };

        Some(Self {
            variable,
            operation: Token::from_node(node.child(1)?, code_bytes)?,
            value: Expression::from_node(node.child(2)?, code_bytes).map(Arc::new)?,
        })
    }
}
impl HasRange for CompoundSetExpression {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.variable.get_range(), self.value.get_range())
    }
}
