//! Implements display traits for expressions.

#[cfg(feature = "raw-values")]
use std::fmt::Write;

#[cfg(feature = "raw-values")]
use super::type_definition::try_generics_to_string;
use crate::{
    impl_print_enum, impl_print_struct, optional_print, print,
    types::{
        ElseIfExpression, Expression, ExpressionWrap, FunctionArguments, FunctionCall,
        FunctionCallInvoked, PrefixExp, Table, TableAccess, TableAccessKey, TableAccessPrefix,
        TableField, TableFieldValue, TableKey, Var,
    },
};
#[cfg(feature = "raw-values")]
use crate::{
    types::{HasRawValue, Number, StringLiteral},
    utils::fix_table_indentation,
};

impl_print_struct!(
    ElseIfExpression,
    { self.else_if_token, print! },
    { self.condition, print! },
    { self.then_token, print! },
    { self.expression, print! }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        match self {
            Expression::ERROR => "*error*".to_string(),
            Expression::Nil(value)
            | Expression::Boolean(value)
            | Expression::Number(Number(value))
            | Expression::String(StringLiteral(value)) => value.get_raw_value(),
            Expression::Function {
                generics,
                parameters,
                returns,
                ..
            } => format!(
                "function{}({}){}",
                try_generics_to_string(generics, true),
                parameters.get_raw_value(),
                returns.as_ref().map_or(String::new(), |returns| format!(
                    ": {}",
                    returns.get_raw_value()
                ))
            ),
            Expression::FunctionCall(value) => value.get_raw_value(),
            Expression::ExpressionWrap(value) => value.expression.get_raw_value(),
            Expression::Var(value) => value.get_raw_value(),
            Expression::Table(value) => value.get_raw_value(),
            Expression::UnaryExpression {
                operator,
                expression,
            } => format!("{}{}", operator.get_raw_value(), expression.get_raw_value()),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => format!(
                "{} {} {}",
                left.get_raw_value(),
                operator.get_raw_value(),
                right.get_raw_value()
            ),
            Expression::Cast {
                expression,
                cast_to,
                ..
            } => format!(
                "{} :: {}",
                expression.get_raw_value(),
                cast_to.get_raw_value()
            ),
            Expression::IfExpression {
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
impl_print_enum!(
    Expression,
    {
        ERROR,
    },
    {
        Nil,
        Boolean,
        Number,
        String,
        FunctionCall,
        ExpressionWrap,
        Var,
        Table,
    },
    {
        {
            Function,
            {
                { function_keyword, print! },
                { generics, optional_print! },
                { opening_parenthesis, print!},
                { parameters, print!},
                { closing_parenthesis, print!},
                { colon, optional_print!},
                { returns, optional_print!},
                { body, print!},
                { end_keyword, print!},
            }
        },
        {
            UnaryExpression,
            {
                { operator, print! },
                { expression, print! },
            }
        },
        {
            BinaryExpression,
            {
                { left, print! },
                { operator, print! },
                { right, print! },
            }
        },
        {
            Cast,
            {
                { expression, print! },
                { operator, print! },
                { cast_to, print! },
            }
        },
        {
            IfExpression,
            {
                { if_token, print! },
                { condition, print! },
                { then_token, print! },
                { if_expression, print! },
                { else_if_expressions, print! },
                { else_token, print! },
                { else_expression, print! },
            }
        },
    }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for Table {
    fn get_raw_value(&self) -> String {
        let len = self.fields.len();
        if len == 0 {
            return "{}".to_string();
        } else if len == 1 {
            return format!("{{ {} }}", self.fields[0].get_raw_value());
        }

        fix_table_indentation(&format!(
            "{{\n{}\n}}",
            self.fields.raw_value_with_separator("\n")
        ))
    }
}
impl_print_struct!(
    Table,
    { self.opening_brackets, print! },
    { self.fields, print! },
    { self.closing_brackets, print! }
);

#[cfg(feature = "raw-values")]
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
impl_print_struct!(
    TableField,
    { self.key, print! },
    { self.equal_or_colon, optional_print! },
    { self.value, print! }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for TableKey {
    fn get_raw_value(&self) -> String {
        match self {
            Self::ERROR => "*error*".to_string(),
            Self::UndefinedNumber(_) | Self::UndefinedString(_) => "".to_string(),
            Self::String(value) => value.get_raw_value(),
            Self::Expression { expression, .. } => format!("[{}]", expression.get_raw_value()),
            Self::Type { r#type, .. } => format!("[{}]", r#type.get_raw_value()),
        }
    }
}
impl_print_enum!(
    TableKey,
    { ERROR, },
    { UndefinedNumber, UndefinedString, String, },
    {
        {
            Expression,
            {
                { open_square_brackets, print! },
                { expression, print! },
                { close_square_brackets, print! },
            }
        },
        {
            Type,
            {
                { open_square_brackets, print! },
                { r#type, print! },
                { close_square_brackets, print! },
            }
        },
    }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for TableFieldValue {
    fn get_raw_value(&self) -> String {
        match self {
            Self::ERROR => "*error*".to_string(),
            Self::Expression(value) => value.get_raw_value(),
            Self::Type(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    TableFieldValue,
    { ERROR, },
    { Expression, Type, },
    {}
);

#[cfg(feature = "raw-values")]
impl HasRawValue for PrefixExp {
    fn get_raw_value(&self) -> String {
        match self {
            PrefixExp::Var(value) => value.get_raw_value(),
            PrefixExp::FunctionCall(value) => value.get_raw_value(),
            PrefixExp::ExpressionWrap(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    PrefixExp,
    {},
    { Var, FunctionCall, ExpressionWrap, },
    {}
);

#[cfg(feature = "raw-values")]
impl HasRawValue for Var {
    fn get_raw_value(&self) -> String {
        match self {
            Self::ERROR => "*error*".to_string(),
            Self::Name(value) => value.get_raw_value(),
            Self::TableAccess(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    Var,
    { ERROR, },
    { Name, TableAccess, },
    {}
);

#[cfg(feature = "raw-values")]
impl HasRawValue for ExpressionWrap {
    fn get_raw_value(&self) -> String {
        format!("({})", self.expression.get_raw_value())
    }
}
impl_print_struct!(
    ExpressionWrap,
    { self.opening_parenthesis, print! },
    { self.expression, print! },
    { self.closing_parenthesis, print! }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for TableAccess {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}",
            self.prefix.get_raw_value(),
            self.accessed_keys
                .iter()
                .fold(String::new(), |mut string, key| {
                    let _ = write!(string, "{}{}", key.get_separator(), key.get_raw_value());
                    string
                }),
        )
    }
}
impl_print_struct!(
    TableAccess,
    { self.prefix, print! },
    { self.accessed_keys, print! }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for TableAccessPrefix {
    fn get_raw_value(&self) -> String {
        match self {
            TableAccessPrefix::Name(value) => value.get_raw_value(),
            TableAccessPrefix::FunctionCall(value) => value.get_raw_value(),
            TableAccessPrefix::ExpressionWrap(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    TableAccessPrefix,
    {},
    { Name, FunctionCall, ExpressionWrap, },
    {}
);

#[cfg(feature = "raw-values")]
impl HasRawValue for TableAccessKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableAccessKey::Expression(value) => value.get_raw_value(),
            TableAccessKey::Name { name, .. } => name.get_raw_value(),
        }
    }
}
#[cfg(feature = "raw-values")]
impl TableAccessKey {
    /// Get the separator that's leading this table access, if it's a name it'll
    /// be a `.`, else it'll be an empty string.
    pub fn get_separator(&self) -> String {
        match self {
            TableAccessKey::Expression(_) => String::new(),
            TableAccessKey::Name { .. } => String::from("."),
        }
    }
}
impl_print_enum!(
    TableAccessKey,
    {},
    { Expression, },
    {
        {
            Name,
            {
                { dot, print! },
                { name, print! },
            }
        },
    }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for FunctionCall {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}",
            self.invoked.get_raw_value(),
            self.arguments.get_raw_value()
        )
    }
}
impl_print_struct!(FunctionCall, { self.invoked, print! }, { self.arguments, print! });

#[cfg(feature = "raw-values")]
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
impl_print_enum!(
    FunctionCallInvoked,
    {},
    { Function, },
    {
        {
            TableMethod,
            {
                { table, print! },
                { colon, print! },
                { method, print! },
            }
        },
    }
);

#[cfg(feature = "raw-values")]
impl HasRawValue for FunctionArguments {
    fn get_raw_value(&self) -> String {
        match self {
            FunctionArguments::String(value) => value.get_raw_value(),
            FunctionArguments::Table(value) => value.get_raw_value(),
            FunctionArguments::List { arguments, .. } => format!("({})", arguments.get_raw_value()),
        }
    }
}
impl_print_enum!(
    FunctionArguments,
    {},
    { String, Table, },
    {
        {
            List,
            {
                { open_parenthesis, print! },
                { arguments, print! },
                { close_parenthesis, print! },
            }
        },
    }
);
