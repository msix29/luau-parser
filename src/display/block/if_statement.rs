//! Implements display traits for if statements

use std::fmt::Display;

use crate::prelude::{HasRawValue, IfStatement};

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for IfStatement {
    fn get_raw_value(&self) -> String {
        todo!()
    }
}
