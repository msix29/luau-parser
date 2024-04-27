//! Implements helper traits for set expressions.

use std::sync::Arc;

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        CompoundSetExpression, Expression, HasRange, List, LuauStatement, PrefixExp, Range,
        SetExpression, Token,
    },
    utils::get_range_from_boundaries,
};

use super::expression::handle_prefix_exp::handle_prefix_exp;

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
                |_, node| {
                    let prefix_exp = handle_prefix_exp(node, code_bytes);
                    match prefix_exp {
                        PrefixExp::Var(var) => var,
                        _ => unreachable!(),
                    }
                },
            ),
            equal: Token::from((node.child_by_field_name("equal").unwrap(), code_bytes)),
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
        let prefix_exp = handle_prefix_exp(node.child(0).unwrap(), code_bytes);
        let variable = match prefix_exp {
            PrefixExp::Var(var) => var,
            _ => unreachable!(),
        };

        Some(Self {
            variable,
            operation: Token::from((node.child(1).unwrap(), code_bytes)),
            value: Arc::new(Expression::from((node.child(2).unwrap(), code_bytes))),
        })
    }
}
impl HasRange for CompoundSetExpression {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.variable.get_range(), self.value.get_range())
    }
}
