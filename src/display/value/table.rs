//! Implements display traits for _[table fields](TableField)_,
//! _[table field values](TableFieldValue)_, _[table keys](TableKey)_, and the
//! _[table values](TableValue)_.

use std::fmt::Display;

use crate::prelude::{HasRawValue, TableField, TableFieldValue, TableKey, TableValue};

impl Display for TableKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TableKey {
    fn get_raw_value(&self) -> String {
        match self {
            TableKey::String(key) => key.to_string(),
            TableKey::Expression {
                open_square_brackets,
                expression,
                close_square_brackets,
            } => format!(
                "{}{}{}",
                open_square_brackets,
                expression.get_raw_value(),
                close_square_brackets
            ),
            TableKey::Type {
                open_square_brackets,
                r#type,
                close_square_brackets,
            } => format!(
                "{}{}{}",
                open_square_brackets,
                r#type.get_raw_value(),
                close_square_brackets
            ),
        }
    }
}

impl Display for TableField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TableField {
    fn get_raw_value(&self) -> String {
        let value = if let Some(value) = &self.value {
            format!(" = {}", value.get_raw_value())
        } else {
            String::from("")
        };
        let r#type = if let Some(r#type) = &self.r#type {
            format!(": {}", r#type.get_raw_value())
        } else {
            String::from("")
        };
        format!("{}{}{}", self.key.get_raw_value(), r#type, value,)
    }
}

impl Display for TableFieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TableFieldValue {
    fn get_raw_value(&self) -> String {
        "".to_string()
    }
}

impl Display for TableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TableValue {
    fn get_raw_value(&self) -> String {
        self.fields
            .items
            .iter()
            .map(|field| (*field).get_raw_value())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
