use std::fmt::Display;

use crate::prelude::{ElseIfExpression, HasRawValue};

impl Display for ElseIfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for ElseIfExpression {
    fn get_raw_value(&self) -> String {
        format!(
            "{} {} {} {}",
            self.else_if_token.get_raw_value(),
            self.condition.get_raw_value(),
            self.then_token.get_raw_value(),
            self.expression.get_raw_value(),
        )
    }
}
