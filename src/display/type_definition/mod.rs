//! Implements display traits for type definition and all related structs and enums.

mod type_value;

use std::fmt::Display;

use crate::prelude::{HasRawValue, Print, TypeDefinition};

impl Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if self.type_name == "any" {
            return "any".to_string();
        }

        let prefix = self
            .export_keyword
            .as_ref()
            .map_or_else(|| "".to_string(), |export| export.get_raw_value());

        let start = if self.type_name.is_empty() {
            String::from("")
        } else {
            format!("type {} = ", self.type_name)
        };

        format!("{}{}{}", prefix, start, self.type_value.get_raw_value())
    }
}

impl Print for TypeDefinition {
    fn print(&self) -> String {
        todo!()
    }
    fn print_leading(&self) -> String {
        todo!()
    }
    fn print_trailing(&self) -> String {
        todo!()
    }
}