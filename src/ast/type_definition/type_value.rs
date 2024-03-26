use std::sync::Arc;
use tree_sitter::Node;

use crate::prelude::{Expression, ExpressionInner, TypeValue};

use super::functions::from_simple_type;

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        //TODO: & and | types.
        TypeValue {
            r#type: Arc::new(Expression::from((
                node,
                from_simple_type(node, code_bytes),
                code_bytes,
            ))),
            ..Default::default()
        }
    }
}
impl From<(&str, Node<'_>)> for TypeValue {
    fn from((name, node): (&str, Node<'_>)) -> Self {
        TypeValue {
            r#type: Arc::new((ExpressionInner::from((name, node)), node).into()),
            ..Default::default()
        }
    }
}
impl From<(Arc<ExpressionInner>, Node<'_>)> for TypeValue {
    fn from((value, node): (Arc<ExpressionInner>, Node<'_>)) -> Self {
        TypeValue {
            r#type: Arc::new(Expression::from((value.clone(), node))),
            ..Default::default()
        }
    }
}
