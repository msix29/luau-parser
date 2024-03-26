use std::fmt::Display;

use crate::prelude::{ExpressionInner, HasRawValue};

impl Display for ExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for ExpressionInner {
    fn get_raw_value(&self) -> String {
        match self {
            ExpressionInner::Nil => "nil".to_string(),
            ExpressionInner::Boolean(value) => value.get_raw_value(),
            ExpressionInner::Number(value) => value.get_raw_value(),
            ExpressionInner::String(value) => value.get_raw_value(),
            ExpressionInner::Function(value) => value.get_raw_value(),
            ExpressionInner::FunctionCall(_) => todo!("function call"),
            ExpressionInner::ExpressionWrap(_) => todo!(),
            ExpressionInner::Var(_) => todo!(),
            ExpressionInner::Table(value) => value.get_raw_value(),
            ExpressionInner::UnaryExpression {
                operator,
                expression,
            } => format!("{}{}", operator.get_raw_value(), expression.get_raw_value()),
            ExpressionInner::BinaryExpression {
                left,
                operator,
                right,
            } => format!(
                "{} {} {}",
                left.get_raw_value(),
                operator.get_raw_value(),
                right.get_raw_value()
            ),

            ExpressionInner::Cast {
                expression,
                operator,
                cast_to,
            } => {
                format!(
                    "{} {} {}",
                    expression,
                    operator.get_raw_value(),
                    cast_to.get_raw_value()
                )
            }
            ExpressionInner::IfExpression {
                if_token,
                condition,
                then_token,
                else_if_expressions,
                else_token,
                else_expression,
            } => {
                format!(
                    "{} {} {} {} {} {}",
                    if_token.get_raw_value(),
                    condition.get_raw_value(),
                    then_token.get_raw_value(),
                    else_if_expressions
                        .iter()
                        .map(|expression| expression.get_raw_value())
                        .collect::<Vec<String>>()
                        .join(" "),
                    else_token.get_raw_value(),
                    else_expression.get_raw_value(),
                )
            }
        }
    }
}
