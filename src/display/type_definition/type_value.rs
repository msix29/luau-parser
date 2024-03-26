use std::fmt::Display;

use crate::prelude::{HasRawValue, TypeValue};

impl Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        let mut main_type = self.r#type.get_raw_value();

        // According to Luau rules, `&` and `|` can't be joined in one type, you must do
        // `( ... & ...) | ...` for it to work, which is why this is an `if-else if` instead
        // of 2 `if` statements.
        if !self.and_types.is_empty() {
            let and_types = self
                .and_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" & ");
            main_type = format!("({} & {})", main_type, and_types)
        } else if !self.or_types.is_empty() {
            let or_types = self
                .or_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" | ");
            main_type = format!("({} | {})", main_type, or_types)
        }

        main_type.to_string()
    }
}
