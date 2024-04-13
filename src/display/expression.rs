//! Implements display traits for expressions.

use crate::{
    impl_print_enum, impl_print_struct, optional_print,
    prelude::{
        ElseIfExpression, Expression, ExpressionWrap, FunctionArguments, FunctionCall,
        FunctionCallInvoked, HasRawValue, PrefixExp, Table, TableAccess, TableAccessKey,
        TableAccessPrefix, TableField, TableFieldValue, TableKey, Var,
    },
    print,
    utils::fix_table_indentation,
};

use super::type_definition::try_generics_to_string;

impl_print_struct!(
    ElseIfExpression,
    { self.else_if_token, print! },
    { self.condition, print! },
    { self.then_token, print! },
    { self.expression, print! }
);

impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        match self {
            Expression::Nil(value)
            | Expression::Boolean(value)
            | Expression::Number(value)
            | Expression::String(value) => value.get_raw_value(),
            Expression::Function {
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
    {},
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
                { opening_parenthesis, print!} ,
                { parameters, print!} ,
                { closing_parenthesis, print!} ,
                { returns, print!} ,
                { body, print!} ,
                { end_keyword, print!} ,
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
impl_print_struct!(
    Table,
    { self.opening_brackets, print! },
    { self.fields, print! },
    { self.closing_brackets, print! }
);

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
impl_print_enum!(
    TableKey,
    {},
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

impl HasRawValue for TableFieldValue {
    fn get_raw_value(&self) -> String {
        match self {
            TableFieldValue::Expression(value) => value.get_raw_value(),
            TableFieldValue::Type(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    TableFieldValue,
    {},
    { Expression, Type, },
    {}
);

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

impl HasRawValue for Var {
    fn get_raw_value(&self) -> String {
        match self {
            Var::Name(value) => value.get_raw_value(),
            Var::TableAccess(value) => value.get_raw_value(),
        }
    }
}
impl_print_enum!(
    Var,
    {},
    { Name, TableAccess, },
    {}
);

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

impl HasRawValue for TableAccess {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}",
            self.prefix.get_raw_value(),
            self.accessed_keys.last().unwrap().get_raw_value()
        )
    }
}
impl_print_struct!(
    TableAccess,
    { self.prefix, print! },
    { self.accessed_keys, print! }
);

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

impl HasRawValue for TableAccessKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableAccessKey::Expression(value) => value.get_raw_value(),
            TableAccessKey::Name { name, .. } => name.get_raw_value(),
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
