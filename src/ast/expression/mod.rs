//! Implements various `From<>` traits for `Expression` and `ExpressionInner`.

mod expression_inner;
pub(crate) mod handle_prefix_exp;

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{Expression, ExpressionInner, HasLocation, Location},
    utils::get_spaces,
};

impl From<(Node<'_>, &[u8])> for Expression {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
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
            spaces_before,
            inner: Arc::new(expression_inner),
            spaces_after,
        }
    }
}
impl From<ExpressionInner> for Expression {
    fn from(expression_inner: ExpressionInner) -> Self {
        Self {
            spaces_before: "".to_string(),
            inner: Arc::new(expression_inner),
            spaces_after: "".to_string(),
        }
    }
}

impl HasLocation for Expression {
    fn get_location(&self) -> Location {
        self.inner.get_location()
    }
}
