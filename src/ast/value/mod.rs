//! Implements helper trait for the _[value enum](Value)_ and loads in other modules for
//! each corresponding entry in the enum.

mod function;
mod simple;
mod table;

use std::fmt::Display;

use tree_sitter::Node;

use crate::prelude::{HasRawValue, SimpleValue, TableValue, TypeDefinition, Value};

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

impl Value {
    pub fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> Vec<(Value, Option<TypeDefinition>)> {
        let mut values = Vec::new();

        for node in nodes_iter {
            match node.kind() {
                "nil" => values.push((Value::from("nil"), None)),
                "boolean" => values.push((Value::from(node.utf8_text(code_bytes).unwrap()), None)),
                "number" => values.push((Value::from(node.utf8_text(code_bytes).unwrap()), None)),
                "string" => values.push((Value::from(node.utf8_text(code_bytes).unwrap()), None)),
                "string_interp" => {
                    values.push((Value::from(node.utf8_text(code_bytes).unwrap()), None))
                }
                "anon_fn" => todo!(),
                "prefixexp" => todo!(),
                "table" => {
                    //TODO:
                    let value = Value::TableValue(TableValue { fields: Vec::new() });
                    values.push((value.clone(), Some(TypeDefinition::from(value))))
                }
                "unexp" => println!("unexp"),
                "binexp" => println!("binexp"),
                "cast" => println!("cast"),
                "ifexp" => println!("ifexp"),
                _ => (),
            }
        }

        values
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::SimpleValue(SimpleValue::default())
    }
}
