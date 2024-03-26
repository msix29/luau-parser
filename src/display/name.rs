use std::fmt::Display;

use crate::prelude::{HasRawValue, NormalizedName, Print};

impl Display for NormalizedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for NormalizedName {
    fn get_raw_value(&self) -> String {
        if let Some(r#type) = &self.r#type {
            format!("{}: {}", self.name, r#type)
        } else {
            self.name.to_string()
        }
    }
}

impl Print for NormalizedName {
    fn print(&self) -> String {
        format!(
            "{}{}{}",
            self.spaces_before,
            self.get_raw_value(),
            self.spaces_after
        )
    }
    fn print_leading(&self) -> String {
        format!("{}{}", self.spaces_before, self.get_raw_value())
    }
    fn print_trailing(&self) -> String {
        format!("{}{}", self.get_raw_value(), self.spaces_after)
    }
}
