use std::fmt::Display;

use crate::prelude::{HasRawValue, TypeValue};

impl Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        todo!()
    }
}
