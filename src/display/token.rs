//! Implements display traits for single tokens.

#[cfg(feature = "raw-values")]
use crate::prelude::HasRawValue;
use crate::prelude::{Print, SingleToken};

#[cfg(feature = "raw-values")]
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
