//! Implements display traits for expressions.

mod else_if_expression;
mod expression_inner;

use std::fmt::Display;

use crate::prelude::{Expression, HasRawValue, Print};

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
