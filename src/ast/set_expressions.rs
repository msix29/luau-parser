//! Implements helper traits for set expressions.

use crate::prelude::{
    AstNode, Expression, ExpressionInner, List, PrefixExp, SetExpression, SingleToken,
};

use super::expression::handle_prefix_exp::handle_prefix_exp;

impl AstNode for SetExpression {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
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
