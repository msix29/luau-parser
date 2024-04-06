//! Implements display traits for expressions.

use crate::prelude::{
    Expression, ExpressionInner, HasRawValue, TableField, TableFieldValue, TableKey, TableValue,
};

use super::type_definition::try_generics;

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
                generics,
                opening_parenthesis,
                closing_parenthesis,
                parameters,
                returns,
                body,
                end_keyword,
            } => format!(
                "function {}({}): {}",
                try_generics(generics),
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
                if_expression,
                else_if_expressions,
                else_token,
                else_expression,
            } => format!(
                "if {} then\
                    \t{}\
                {}
                else\
                    \t{}",
                condition.get_raw_value(),
                if_expression.get_raw_value(),
                else_if_expressions
                    .iter()
                    .map(|expression| format!(
                        "elseif {} then\
                            \t{}",
                        expression.condition.get_raw_value(),
                        expression.expression.get_raw_value()
                    ))
                    .collect::<Vec<String>>()
                    .join("\n"),
                else_expression.get_raw_value()
            ),
        }
    }
}

impl HasRawValue for TableValue {
    fn get_raw_value(&self) -> String {
        let len = self.fields.items.len();
        if len == 0 {
            return "{}".to_string();
        } else if len == 1 {
            return format!("{{ {} }}", self.fields.items[0].get_raw_value());
        }

        "".to_string()
    }
}

impl HasRawValue for TableField {
    fn get_raw_value(&self) -> String {
        let key = self.key.get_raw_value();
        
        if key == "" {
            self.value.get_raw_value()
        } else {
            let equal_or_colon = self.equal_or_colon.unwrap().get_raw_value();
            if equal_or_colon == ":" {
                format!("{}{} {}", key, ":", self.value.get_raw_value())
            } else {
                format!("{} {} {}", key, "=", self.value.get_raw_value())
            }
        }
    }
}
impl HasRawValue for TableKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => "".to_string(),
            TableKey::String(value) => value.get_raw_value(),
            TableKey::Expression {
                open_square_brackets,
                expression,
                close_square_brackets,
            } => format!("[{}]", expression.get_raw_value()),
            TableKey::Type {
                open_square_brackets,
                r#type,
                close_square_brackets,
            } => format!("[{}]", r#type.get_raw_value()),
        }
    }
}
impl HasRawValue for TableFieldValue {
    fn get_raw_value(&self) -> String {
        match self {
            TableFieldValue::Expression(value) => value.get_raw_value(),
            TableFieldValue::Type(value) => value.get_raw_value(),
        }
    }
}
