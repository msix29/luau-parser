//! Implements display traits for single tokens.

use std::fmt::{Debug, Display};

use crate::prelude::{HasRawValue, Print, SingleToken};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
impl Display for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl Debug for SingleToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Word ({})", &self.get_raw_value()))
    }
}

impl Print for SingleToken {
    fn print(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.word, self.spaces_after)
    }
    fn print_leading(&self) -> String {
        format!("{}{}", self.spaces_before, self.word)
    }
    fn print_trailing(&self) -> String {
        format!("{}{}", self.word, self.spaces_after)
    }
}
