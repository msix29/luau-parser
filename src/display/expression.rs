//! Implements display traits for expressions.

use crate::{
    impl_print,
    prelude::{
        Expression, ExpressionInner, ExpressionWrap, FunctionArguments, FunctionCall,
        FunctionCallInvoked, HasRawValue, PrefixExp, Table, TableAccess, TableAccessKey,
        TableAccessPrefix, TableField, TableFieldValue, TableKey, Var,
    },
    utils::fix_table_indentation,
};

use super::type_definition::try_generics_to_string;

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
                generics,
                parameters,
                returns,
                ..
            } => format!(
                "function {}({}): {}",
                try_generics_to_string(generics),
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
                cast_to,
                ..
            } => format!(
                "{} :: {}",
                expression.get_raw_value(),
                cast_to.get_raw_value()
            ),
            ExpressionInner::IfExpression {
                condition,
                if_expression,
                else_if_expressions,
                else_expression,
                ..
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

impl HasRawValue for Table {
    fn get_raw_value(&self) -> String {
        let len = self.fields.items.len();
        if len == 0 {
            return "{}".to_string();
        } else if len == 1 {
            return format!("{{ {} }}", self.fields.items[0].get_raw_value());
        }

        fix_table_indentation(&format!("{{\n{}\n}}", self.fields.get_raw_value()))
    }
}

impl HasRawValue for TableField {
    fn get_raw_value(&self) -> String {
        let key = self.key.get_raw_value();

        if key.is_empty() {
            self.value.get_raw_value()
        } else {
            let equal_or_colon = self.equal_or_colon.as_ref().unwrap().get_raw_value();
            if equal_or_colon == ":" {
                format!("{}: {}", key, self.value.get_raw_value())
            } else {
                format!("{} = {}", key, self.value.get_raw_value())
            }
        }
    }
}
impl HasRawValue for TableKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => "".to_string(),
            TableKey::String(value) => value.get_raw_value(),
            TableKey::Expression { expression, .. } => format!("[{}]", expression.get_raw_value()),
            TableKey::Type { r#type, .. } => format!("[{}]", r#type.get_raw_value()),
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

impl HasRawValue for PrefixExp {
    fn get_raw_value(&self) -> String {
        match self {
            PrefixExp::Var(value) => value.get_raw_value(),
            PrefixExp::FunctionCall(value) => value.get_raw_value(),
            PrefixExp::ExpressionWrap(value) => value.get_raw_value(),
        }
    }
}
impl HasRawValue for Var {
    fn get_raw_value(&self) -> String {
        match self {
            Var::Name(value) => value.get_raw_value(),
            Var::TableAccess(value) => value.get_raw_value(),
        }
    }
}

impl HasRawValue for ExpressionWrap {
    fn get_raw_value(&self) -> String {
        format!("({})", self.expression.get_raw_value())
    }
}

impl HasRawValue for TableAccess {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}",
            self.prefix.get_raw_value(),
            self.accessed_keys.last().unwrap().get_raw_value()
        )
    }
}
impl HasRawValue for TableAccessPrefix {
    fn get_raw_value(&self) -> String {
        match self {
            TableAccessPrefix::Name(value) => value.get_raw_value(),
            TableAccessPrefix::FunctionCall(value) => value.get_raw_value(),
            TableAccessPrefix::ExpressionWrap(value) => value.get_raw_value(),
        }
    }
}
impl HasRawValue for TableAccessKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableAccessKey::Expression(value) => value.get_raw_value(),
            TableAccessKey::Name { name, .. } => name.get_raw_value(),
        }
    }
}

impl HasRawValue for FunctionCall {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}",
            self.invoked.get_raw_value(),
            self.arguments.get_raw_value()
        )
    }
}
impl HasRawValue for FunctionCallInvoked {
    fn get_raw_value(&self) -> String {
        match self {
            FunctionCallInvoked::Function(value) => value.get_raw_value(),
            FunctionCallInvoked::TableMethod { table, method, .. } => {
                format!("{}:{}", table.get_raw_value(), method.get_raw_value())
            }
        }
    }
}
impl HasRawValue for FunctionArguments {
    fn get_raw_value(&self) -> String {
        match self {
            FunctionArguments::String(value) => value.get_raw_value(),
            FunctionArguments::Table(value) => value.get_raw_value(),
            FunctionArguments::List { arguments, .. } => format!("({})", arguments.get_raw_value()),
        }
    }
}
impl_print!(FunctionCall);
