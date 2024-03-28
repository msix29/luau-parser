//! Implements display traits for _[function parameters](FunctionParameter)_,
//! _[function returns](FunctionReturn)_, and _[value](FunctionValue)_ in the
//! _[value enum](crate::prelude::Value)_.

use std::fmt::Display;

use crate::prelude::{FunctionParameter, FunctionReturn, FunctionValue, HasRawValue};

impl Display for FunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for FunctionParameter {
    fn get_raw_value(&self) -> String {
        format!("{}: {}", self.name, self.r#type.get_raw_value())
    }
}

impl Display for FunctionReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for FunctionReturn {
    fn get_raw_value(&self) -> String {
        self.r#type.get_raw_value().to_string()
    }
}

impl Display for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for FunctionValue {
    fn get_raw_value(&self) -> String {
        todo!()
        // format!(
        //     "({}) -> ({})",
        //     self.parameters
        //         .iter()
        //         .map(|parameter| parameter.get_raw_value())
        //         .collect::<Vec<String>>()
        //         .join(", "),
        //     self.returns
        //         .iter()
        //         .map(|r#return| r#return.get_raw_value())
        //         .collect::<Vec<String>>()
        //         .join(", ")
        // )
    }
}
