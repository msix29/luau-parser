//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! _[type definitions](TypeDefinition)_ from _[nodes](Node)_, to implementing helper
//! traits for both _[type definitions](TypeDefinition)_ and _[type values](TypeValue)_.
//!

use std::fmt::Display;
use tree_sitter::Node;

use crate::prelude::{
    AstNode, FunctionParameter, FunctionReturn, FunctionValue, HasRawValue, NormalizedName,
    PossibleValues, TableField, TableKey, TableValue, TypeDefinition, TypeValue,
};

fn from_singleton_type(node: Node, code_bytes: &[u8]) -> PossibleValues {
    match node.kind() {
        "string" => PossibleValues::from(node.utf8_text(code_bytes).unwrap()),
        "name" => PossibleValues::from("<other value here>"), // TODO: Look for it.
        "false" => PossibleValues::from("false"),
        "true" => PossibleValues::from("true"),
        _ => PossibleValues::from("any"), // Should never be matched when done.
    }
}

fn build_table_type(node: Node, code_bytes: &[u8]) -> TableValue {
    let mut fields: Vec<TableField> = Vec::new();
    let Some(passed_node) = node.child_by_field_name("fields") else {
        return TableValue {
            fields: Box::new(fields),
        };
    };
    match passed_node.kind() {
        "type" => {
            fields.push(TableField {
                key: Box::new(TableKey::String("[number]".to_string())),
                r#type: Box::new(TypeDefinition::from((passed_node, code_bytes, false))),
                value: None,
            });
        }
        "propList" => {
            let mut i = 0;
            let length = passed_node.child_count();
            while i < length {
                let node = passed_node.child(i).unwrap().child(0).unwrap();
                i += 1;
                match node.kind() {
                    "fieldsep" => continue,
                    "tableProperty" => {
                        fields.push(TableField {
                            key: Box::new(TableKey::String(
                                node.child_by_field_name("key")
                                    .unwrap()
                                    .utf8_text(code_bytes)
                                    .unwrap()
                                    .to_string(),
                            )),
                            r#type: Box::new(TypeDefinition::from((
                                node.child_by_field_name("type").unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                        });
                    }
                    "tableIndexer" => {
                        fields.push(TableField {
                            key: Box::new(TableKey::Type(TypeDefinition::from((
                                node.child_by_field_name("key").unwrap(),
                                code_bytes,
                                false,
                            )))),
                            r#type: Box::new(TypeDefinition::from((
                                node.child_by_field_name("type").unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                        });
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    TableValue {
        fields: Box::new(fields),
    }
}

fn build_function_parameters(node: Node, code_bytes: &[u8]) -> Vec<FunctionParameter> {
    let mut parameters = Vec::new();

    if let Some(parameters_node) = node.child_by_field_name("parameters") {
        for i in 0..parameters_node.child_count() {
            let parameter = parameters_node.child(i).unwrap();
            let normalized_name = NormalizedName::from((parameter, code_bytes));
            parameters.push(FunctionParameter {
                name: normalized_name.name,
                is_variadic: false,
                r#type: normalized_name
                    .r#type
                    .unwrap_or(Box::<TypeDefinition>::default()),
            });
        }
    }

    parameters
}

fn build_function_returns(node: Node, code_bytes: &[u8]) -> Vec<FunctionReturn> {
    let mut returns = Vec::new();

    match node.kind() {
        "(" => {
            for i in 0..node.child_count() {
                returns.push(FunctionReturn {
                    r#type: Box::new(TypeDefinition::from((node.child(i).unwrap(), code_bytes, false))),
                    is_variadic: false,
                });
            }
        }
        "type" => returns.push(FunctionReturn {
            r#type: Box::new(TypeDefinition::from((node, code_bytes, false))),
            is_variadic: false,
        }),
        _ => (),
    }

    returns
}

fn build_function_type(node: Node, code_bytes: &[u8]) -> FunctionValue {
    let parameters = if let Some(node) = node.child_by_field_name("parameters") {
        build_function_parameters(node, code_bytes)
    } else {
        Vec::new()
    };

    FunctionValue {
        parameters: Box::new(parameters),
        returns: Box::new(build_function_returns(
            node.child_by_field_name("returns")
                .unwrap()
                .child(0)
                .unwrap(),
            code_bytes,
        )),
    }
}

fn from_simple_type(node: Node, code_bytes: &[u8]) -> PossibleValues {
    match node.kind() {
        "singleton" => from_singleton_type(node, code_bytes),
        "namedtype" => PossibleValues::from(node.utf8_text(code_bytes).unwrap()), //TODO: indexing from a table.
        "typeof" => PossibleValues::from("typeof<T>(...)"), //TODO: typeof(<expression>)
        "tableType" => PossibleValues::TableValue(build_table_type(node, code_bytes)),
        "simpleType" => from_simple_type(node.child(0).unwrap(), code_bytes),
        "functionType" => PossibleValues::FunctionValue(build_function_type(node, code_bytes)),
        "wraptype" => from_simple_type(node.child(1).unwrap(), code_bytes),
        _ => PossibleValues::from("any"), // Should never be matched when done.
    }
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        let mut main_type = self.r#type.get_raw_value();

        // According to Luau rules, `&` and `|` can't be joined in one type, you must do
        // `( ... & ...) | ...` for it to work, which is why this is an `if-else if` instead
        // of 2 `if` statements.
        if !self.and_types.is_empty() {
            let and_types = self
                .and_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" & ");
            main_type = format!("({} & {})", main_type, and_types)
        } else if !self.or_types.is_empty() {
            let or_types = self
                .or_types
                .iter()
                .map(|r#type| r#type.get_raw_value())
                .collect::<Vec<String>>()
                .join(" | ");
            main_type = format!("({} | {})", main_type, or_types)
        }

        main_type.to_string()
    }
}

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((simple_type, code_bytes): (Node<'_>, &[u8])) -> Self {
        // let simple_type = simple_type.child_by_field_name("simpleType").unwrap();

        TypeValue {
            r#type: Box::new(from_simple_type(simple_type, code_bytes)),
            ..Default::default()
        }
    }
}
impl From<&str> for TypeValue {
    fn from(name: &str) -> Self {
        TypeValue {
            r#type: Box::new(PossibleValues::from(name)),
            ..Default::default()
        }
    }
}
impl From<PossibleValues> for TypeValue {
    fn from(value: PossibleValues) -> Self {
        TypeValue {
            r#type: Box::new(value),
            ..Default::default()
        }
    }
}

impl Default for TypeDefinition {
    fn default() -> Self {
        TypeDefinition {
            type_name: "any".to_string(),
            is_exported: false,
            type_value: Box::<TypeValue>::default(),
        }
    }
}

impl Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if self.type_name == "any" {
            return "any".to_string();
        }

        let prefix = if self.is_exported { "export " } else { "" };
        let start = if self.type_name.is_empty() {
            String::from("")
        } else {
            format!("type {} = ", self.type_name)
        };

        format!("{}{}{}", prefix, start, self.type_value.get_raw_value())
    }
}

impl AstNode for TypeDefinition {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _cursor: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        if node.kind() != "typeDeclaration" {
            return None;
        }

        Some(vec![TypeDefinition::from((node, code_bytes, true))])
    }
}

impl From<(Node<'_>, &[u8], bool)> for TypeDefinition {
    fn from((node, code_bytes, is_definition): (Node, &[u8], bool)) -> Self {
        if is_definition {
            TypeDefinition {
                type_name: node
                    .child_by_field_name("typeName")
                    .unwrap()
                    .utf8_text(code_bytes)
                    .unwrap()
                    .to_string(),
                is_exported: node.child_by_field_name("export").is_some(),
                type_value: Box::new(TypeValue::from((
                    node.child_by_field_name("type").unwrap(),
                    code_bytes,
                ))),
            }
        } else {
            TypeDefinition {
                type_name: "".to_string(),
                is_exported: false,
                type_value: Box::new(TypeValue::from((node, code_bytes))),
            }
        }
    }
}

impl From<&str> for TypeDefinition {
    fn from(type_name: &str) -> Self {
        TypeDefinition {
            type_name: type_name.to_string(),
            is_exported: false,
            type_value: Box::new(TypeValue::from(type_name)),
        }
    }
}

impl From<PossibleValues> for TypeDefinition {
    fn from(value: PossibleValues) -> Self {
        TypeDefinition {
            type_name: "".to_string(),
            is_exported: false,
            type_value: Box::new(TypeValue::from(value)),
        }
    }
}
