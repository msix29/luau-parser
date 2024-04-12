//! Implements display traits for single tokens.

use crate::prelude::{HasRawValue, Print, SingleToken};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
impl Print for SingleToken {
    fn print(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.word, self.spaces_after)
    }
}
