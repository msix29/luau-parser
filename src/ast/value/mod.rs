mod simple;
mod table;
mod function;

use std::fmt::Display;

pub use simple::*;
pub use table::*;
pub use function::*;

use super::HasRawValue;

#[derive(Clone, Debug)]
pub enum Value {
    SimpleValue(SimpleValue),
    FunctionValue(FunctionValue),
    TableValue(TableValue)
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for Value {
    fn get_raw_value(&self) -> String {
        match self {
            Value::SimpleValue(value) => value.get_raw_value(),
            Value::FunctionValue(value) => value.get_raw_value(),
            Value::TableValue(value) => value.get_raw_value(),
        }
    }
}
