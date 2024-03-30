//! Implements helper traits for set expressions.

use tree_sitter::{Node, TreeCursor};

use crate::{
    prelude::{
        CompoundSetExpression, Expression, ExpressionInner, HasLocation, List, Location, LuauStatement, PrefixExp, SetExpression, SingleToken
    },
    utils::get_location_from_boundaries,
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

        Some(SetExpression {
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
            equal: SingleToken::from((node.child_by_field_name("equal").unwrap(), code_bytes)),
            values: ExpressionInner::from_nodes(
                node.children_by_field_name("value", &mut node.walk()),
                code_bytes,
            )
            .to::<Expression>(),
        })
    }
}
impl HasLocation for SetExpression {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.variables.items.last().unwrap().get_location(),
            self.values.items.last().unwrap().get_location(),
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

        Some(CompoundSetExpression {
            variable,
            operation: SingleToken::from((node.child(1).unwrap(), code_bytes)),
            value: Expression::from((node.child(2).unwrap(), code_bytes)),
        })
    }
}
impl HasLocation for CompoundSetExpression {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(self.variable.get_location(), self.value.get_location())
    }
}
