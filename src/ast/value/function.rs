use std::fmt::Display;

use crate::ast::{type_definition::TypeDefinition, HasRawValue};

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    name: String,
    r#type: TypeDefinition,
}
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

#[derive(Clone, Debug)]
pub struct FunctionReturn {
    r#type: TypeDefinition,
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

#[derive(Clone, Debug)]
pub struct FunctionValue {
    pub parameters: &'static [FunctionParameter],
    pub returns: &'static [FunctionReturn],
}
impl Display for FunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for FunctionValue {
    fn get_raw_value(&self) -> String {
        format!(
            "({}) -> ({})",
            self.parameters
                .iter()
                .map(|parameter| parameter.get_raw_value())
                .collect::<Vec<String>>()
                .join(", "),
            self.returns
                .iter()
                .map(|r#return| r#return.get_raw_value())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
