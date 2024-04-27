//! Implements display traits for single tokens.

#[cfg(feature = "raw-values")]
use crate::prelude::HasRawValue;
use crate::prelude::{Print, Token};

#[cfg(feature = "raw-values")]
impl HasRawValue for Token {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
impl Print for Token {
    fn print(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.word, self.spaces_after)
    }
}
