mod expression_inner;
mod else_if_expression;

use std::{fmt::Display, sync::Arc};

use tree_sitter::Node;

use crate::{
    prelude::{Expression, ExpressionInner, HasRawValue, Print},
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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        self.inner.get_raw_value()
    }
}
impl Print for Expression {
    fn print(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.inner, self.spaces_after)
    }
    fn print_leading(&self) -> String {
        format!("{}{}", self.spaces_before, self.inner)
    }
    fn print_trailing(&self) -> String {
        format!("{}{}", self.inner, self.spaces_after)
    }
}
