mod expression_inner;
mod handle_prefix_exp;

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{Expression, ExpressionInner},
    utils::{get_location, get_spaces},
};

impl From<(Node<'_>, &[u8])> for Expression {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            location: get_location(node),
            spaces_before,
            inner: Arc::new(ExpressionInner::from((node, code_bytes))),
            spaces_after,
        }
    }
}
impl From<(Node<'_>, ExpressionInner, &[u8])> for Expression {
    fn from((node, expression_inner, code_bytes): (Node<'_>, ExpressionInner, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            location: get_location(node),
            spaces_before,
            inner: Arc::new(expression_inner),
            spaces_after,
        }
    }
}
impl From<(ExpressionInner, Node<'_>)> for Expression {
    fn from((expression_inner, node): (ExpressionInner, Node<'_>)) -> Self {
        Self {
            inner: Arc::new(expression_inner),
            location: get_location(node),
            ..Default::default()
        }
    }
}
impl From<(Arc<ExpressionInner>, Node<'_>)> for Expression {
    fn from((expression_inner, node): (Arc<ExpressionInner>, Node<'_>)) -> Self {
        Self {
            inner: expression_inner,
            location: get_location(node),
            ..Default::default()
        }
    }
}
