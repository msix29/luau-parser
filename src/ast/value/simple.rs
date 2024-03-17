use std::fmt::Display;

use crate::ast::HasRawValue;

#[derive(Clone, Debug, Default)]
pub struct SimpleValue {
    pub value: String,
}

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
