mod function;
mod simple;
mod table;

use std::fmt::Display;

use crate::prelude::{Value, HasRawValue, SimpleValue};

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

// impl From<Node<'_>> for Value {
//     fn from(value: Node) -> Self {
//         Value::SimpleValue(SimpleValue::default())
//     }
// }
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::SimpleValue(SimpleValue { value })
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::SimpleValue(SimpleValue {
            value: value.to_string(),
        })
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::SimpleValue(SimpleValue::default())
    }
}
