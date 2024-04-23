//! Local and global functions.

#[cfg(feature = "raw-values")]
use super::type_definition::try_generics_to_string;
#[cfg(feature = "raw-values")]
use crate::types::HasRawValue;
use crate::{
    impl_print_enum, impl_print_struct, optional_print, print,
    types::{GlobalFunction, GlobalFunctionName, LocalFunction},
};

#[cfg(feature = "raw-values")]
impl HasRawValue for LocalFunction {
    fn get_raw_value(&self) -> String {
        let return_type = if let Some(return_type) = &self.returns {
            format!(": {}", return_type.get_raw_value())
        } else {
            String::new()
        };

        format!(
            "function {}{}({}){}",
            self.function_name.get_raw_value(),
            try_generics_to_string(&self.generics, false),
            self.parameters.get_raw_value(),
            return_type,
        )
    }
}

#[cfg(feature = "raw-values")]
impl HasRawValue for GlobalFunction {
    fn get_raw_value(&self) -> String {
        let return_type = if let Some(return_type) = &self.returns {
            format!(": {}", return_type.get_raw_value())
        } else {
            String::new()
        };

        format!(
            "function {}{}({}){}",
            self.function_name.get_raw_value(),
            try_generics_to_string(&self.generics, false),
            self.parameters.get_raw_value(),
            return_type,
        )
    }
}

#[cfg(feature = "raw-values")]
impl HasRawValue for GlobalFunctionName {
    fn get_raw_value(&self) -> String {
        match self {
            GlobalFunctionName::SimpleName(name) => name.get_raw_value(),
            GlobalFunctionName::Table {
                table,
                keys,
                method,
            } => {
                let method = if let Some(method) = method {
                    format!(":{}", method.get_raw_value())
                } else {
                    String::new()
                };

                format!(
                    "{}{}{}",
                    table.get_raw_value(),
                    keys.get_raw_value(),
                    method
                )
            }
        }
    }
}

impl_print_struct!(
    LocalFunction,
    { self.local_keyword, print! },
    { self.function_keyword, print! },
    { self.function_name, print! },
    { self.generics, optional_print! },
    { self.opening_parenthesis, print! },
    { self.parameters, print! },
    { self.closing_parenthesis, print! },
    { self.returns, optional_print! },
    { self.body, print! },
    { self.end_keyword, print! }
);

impl_print_enum!(
    GlobalFunctionName,
    {},
    { SimpleName, },
    {
        {
            Table,
            {
                { table, print! },
                { keys, print! },
                { method, optional_print! },
            }
        },
    }
);

impl_print_struct!(
    GlobalFunction,
    { self.function_keyword, print! },
    { self.function_name, print! },
    { self.generics, optional_print! },
    { self.opening_parenthesis, print! },
    { self.parameters, print! },
    { self.closing_parenthesis, print! },
    { self.returns, optional_print! },
    { self.body, print! },
    { self.end_keyword, print! }
);
