//! Implements display traits for single tokens.

use crate::prelude::{HasRawValue, SingleToken};

impl HasRawValue for SingleToken {
    fn get_raw_value(&self) -> String {
        self.word.to_string()
    }
}
