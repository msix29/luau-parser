//! Implements display traits for function parameters.

use crate::prelude::{FunctionParameter, HasRawValue};

impl HasRawValue for FunctionParameter {
    fn get_raw_value(&self) -> String {
        if let Some(r#type) = self.r#type {
            format!("{}: {}", self.name.get_raw_value(), r#type.get_raw_value())
        } else {
            self.name.get_raw_value()
        }
    }
}
