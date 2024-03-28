//! Implements display traits for _[simple values](SimpleValue)_.

use std::fmt::Display;

use crate::prelude::{HasRawValue, SimpleValue};

impl Display for SimpleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for SimpleValue {
    fn get_raw_value(&self) -> String {
        self.value.to_string()
    }
}
