//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! _[type definitions](TypeDefinition)_ from _[nodes](Node)_, to implementing helper
//! traits for both _[type definitions](TypeDefinition)_ and _[type values](TypeValue)_.
//!

mod type_value;

use std::fmt::Display;

use crate::prelude::{HasRawValue, TypeDefinition};

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
