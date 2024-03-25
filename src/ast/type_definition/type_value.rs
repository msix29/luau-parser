use std::{fmt::Display, sync::Arc};
use tree_sitter::Node;

use crate::prelude::{Expression, ExpressionInner, HasRawValue, TypeValue};

use super::functions::from_simple_type;

impl Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        let mut main_type = self.r#type.get_raw_value();

        // According to Luau rules, `&` and `|` can't be joined in one type, you must do
        // `( ... & ...) | ...` for it to work, which is why this is an `if-else if` instead
        // of 2 `if` statements.
        if !self.and_types.is_empty() {
            let and_types = self
                .and_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" & ");
            main_type = format!("({} & {})", main_type, and_types)
        } else if !self.or_types.is_empty() {
            let or_types = self
                .or_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" | ");
            main_type = format!("({} | {})", main_type, or_types)
        }

        main_type.to_string()
    }
}

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
