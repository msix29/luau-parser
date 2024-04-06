//! Implements display traits for expressions.

use crate::prelude::{Expression, ExpressionInner, HasRawValue};

impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        self.inner.get_raw_value()
    }
}

impl HasRawValue for ExpressionInner {
    fn get_raw_value(&self) -> String {
        match self {
            ExpressionInner::Nil(value)
            | ExpressionInner::Boolean(value)
            | ExpressionInner::Number(value)
            | ExpressionInner::String(value) => value.get_raw_value(),
            ExpressionInner::Function {
                function_keyword,
                opening_parenthesis,
                closing_parenthesis,
                parameters,
                returns,
                body,
                end_keyword,
            } => format!(
                "function ({}): {}",
                parameters.get_raw_value(),
                returns.get_raw_value()
            ),
            ExpressionInner::FunctionCall(value) => value.get_raw_value(),
            ExpressionInner::ExpressionWrap(value) => value.expression.get_raw_value(),
            ExpressionInner::Var(value) => value.get_raw_value(),
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
            } => format!(
                "{} :: {}",
                expression.get_raw_value(),
                cast_to.get_raw_value()
            ),
            ExpressionInner::IfExpression {
                if_token,
                condition,
                then_token,
                else_if_expressions,
                else_token,
                else_expression,
            } => todo!(),
        }
    }
}
