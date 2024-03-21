//! Implements helper trait for the _[value enum](Value)_ and loads in other modules for
//! each corresponding entry in the enum.

mod function;
mod simple;
mod table;

use std::fmt::Display;

use tree_sitter::Node;

use crate::prelude::{
    HasRawValue, PossibleValues, SimpleValue, SingleToken, TableValue, TypeDefinition, Value,
};

impl Display for PossibleValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for PossibleValues {
    fn get_raw_value(&self) -> String {
        match self {
            PossibleValues::SimpleValue(value) => value.get_raw_value(),
            PossibleValues::FunctionValue(value) => value.get_raw_value(),
            PossibleValues::TableValue(value) => value.get_raw_value(),
        }
    }
}

// impl From<Node<'_>> for Value {
//     fn from(value: Node) -> Self {
//         Value::SimpleValue(SimpleValue::default())
//     }
// }
impl From<String> for PossibleValues {
    fn from(value: String) -> Self {
        PossibleValues::SimpleValue(SimpleValue { value })
    }
}
impl From<&str> for PossibleValues {
    fn from(value: &str) -> Self {
        PossibleValues::SimpleValue(SimpleValue {
            value: value.to_string(),
        })
    }
}

impl Default for PossibleValues {
    fn default() -> Self {
        PossibleValues::SimpleValue(SimpleValue::default())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value {
            value: Box::new(PossibleValues::SimpleValue(SimpleValue { value })),
            operator: None,
            r#type: None,
        }
    }
}
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value {
            value: Box::new(PossibleValues::SimpleValue(SimpleValue {
                value: value.to_string(),
            })),
            operator: None,
            r#type: None,
        }
    }
}

impl Value {
    pub fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> Vec<Value> {
        let mut values = Vec::new();

        for node in nodes_iter {
            match node.kind() {
                "nil" => values.push(Value::from("nil")),
                "boolean" => values.push(Value::from(node.utf8_text(code_bytes).unwrap())),
                "number" => values.push(Value::from(node.utf8_text(code_bytes).unwrap())),
                "string" => values.push(Value::from(node.utf8_text(code_bytes).unwrap())),
                "string_interp" => values.push(Value::from(node.utf8_text(code_bytes).unwrap())),
                "anon_fn" => todo!(),
                "prefixexp" => todo!(),
                "table" => {
                    //TODO: Fill it
                    values.push(Value {
                        value: Box::new(PossibleValues::TableValue(TableValue {
                            fields: Box::new(Vec::new()),
                        })),
                        operator: None,
                        r#type: None,
                    })
                }
                "unexp" => println!("unexp"),
                "binexp" => println!("binexp"),
                "cast" => {
                    let temp_result = Value::from_nodes(
                        node.children_by_field_name("arg", &mut node.walk()),
                        code_bytes,
                    );
                    let result = temp_result.iter().map(|value| Value {
                        value: value.value.clone(),
                        r#type: Some(Box::new(TypeDefinition::from((
                            node.child_by_field_name("cast").unwrap(),
                            code_bytes,
                            false,
                        )))),
                        operator: Some(SingleToken::from((
                            node.child_by_field_name("op").unwrap(),
                            code_bytes,
                        ))),
                    });
                    values.extend(result);
                }
                "ifexp" => println!("ifexp"),
                _ => (),
            }
        }

        values
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for Value {
    fn get_raw_value(&self) -> String {
        if let Some(r#type) = &self.r#type {
            format!(
                "{}{}{}",
                self.value.get_raw_value(),
                self.operator.as_ref().unwrap(), // If type exists, operator will 100% exist too.
                r#type.get_raw_value()
            )
        } else {
            self.value.get_raw_value()
        }
    }
}
