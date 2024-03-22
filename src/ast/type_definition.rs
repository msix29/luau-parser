//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! _[type definitions](TypeDefinition)_ from _[nodes](Node)_, to implementing helper
//! traits for both _[type definitions](TypeDefinition)_ and _[type values](TypeValue)_.
//!

use std::fmt::Display;
use tree_sitter::Node;

use crate::prelude::{
    AstNode, Expression, ExpressionInner, FunctionParameter, FunctionReturn, FunctionValue,
    HasRawValue, NormalizedName, SingleToken, TableField, TableKey, TableValue, TypeDefinition,
    TypeValue,
};

fn from_singleton_type(node: Node, code_bytes: &[u8]) -> ExpressionInner {
    match node.kind() {
        "string" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()),
        "name" => ExpressionInner::from("<other value here>"), // TODO: Look for it.
        "false" => ExpressionInner::from("false"),
        "true" => ExpressionInner::from("true"),
        _ => ExpressionInner::from("any"), // Should never be matched when done.
    }
}

fn build_table_type(node: Node, code_bytes: &[u8]) -> TableValue {
    let Some(fields_list) = node
        .child_by_field_name("fields")
        .map(|node| node.child(0).unwrap())
    else {
        return TableValue {
            fields: Box::<Vec<TableField>>::default(),
        };
    };
    let separators = fields_list
        .children_by_field_name("sep", &mut node.walk())
        .collect::<Vec<Node>>();
    let mut table_fields: Vec<TableField> = Vec::new();
    match fields_list.kind() {
        "type" => {
            table_fields.push(TableField {
                key: Box::new(TableKey::String("[number]".to_string())),
                equal_or_colon: None,
                r#type: Box::new(TypeDefinition::from((fields_list, code_bytes, false))),
                value: None,
                separator: None,
            });
        }
        "propList" => {
            for (i, field) in fields_list
                .children_by_field_name("field", &mut fields_list.walk())
                .enumerate()
            {
                let separator = separators
                    .get(i)
                    .map(|separator| SingleToken::from((*separator, code_bytes)));

                match field.kind() {
                    "tableProperty" => {
                        table_fields.push(TableField {
                            key: Box::new(TableKey::String(
                                field
                                    .child(0)
                                    .unwrap()
                                    .utf8_text(code_bytes)
                                    .unwrap()
                                    .to_string(),
                            )),
                            equal_or_colon: Some(SingleToken::from((
                                field.child(1).unwrap(),
                                code_bytes,
                            ))),
                            r#type: Box::new(TypeDefinition::from((
                                field.child(2).unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                            separator,
                        });
                    }
                    "tableIndexer" => {
                        table_fields.push(TableField {
                            key: Box::new(TableKey::Type {
                                open_square_brackets: SingleToken::from((
                                    field.child(0).unwrap(),
                                    code_bytes,
                                )),
                                r#type: Box::new(TypeDefinition::from((
                                    field.child(1).unwrap(),
                                    code_bytes,
                                    false,
                                ))),
                                close_square_brackets: SingleToken::from((
                                    field.child(2).unwrap(),
                                    code_bytes,
                                )),
                            }),
                            equal_or_colon: Some(SingleToken::from((
                                field.child(3).unwrap(),
                                code_bytes,
                            ))),
                            r#type: Box::new(TypeDefinition::from((
                                field.child(4).unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                            separator,
                        });
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    TableValue {
        fields: Box::new(table_fields),
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
                    r#type: Box::new(TypeDefinition::from((
                        node.child(i).unwrap(),
                        code_bytes,
                        false,
                    ))),
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

fn from_simple_type(node: Node, code_bytes: &[u8]) -> ExpressionInner {
    match node.kind() {
        "singleton" => from_singleton_type(node, code_bytes),
        "namedtype" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()), //TODO: indexing from a table.
        "typeof" => ExpressionInner::from("typeof<T>(...)"), //TODO: typeof(<expression>)
        "tableType" => ExpressionInner::Table(build_table_type(node, code_bytes)),
        "functionType" => ExpressionInner::Function(build_function_type(node, code_bytes)),
        "wraptype" => from_simple_type(node.child(1).unwrap(), code_bytes),
        // "untype"
        _ => ExpressionInner::from("any"), // Should never be matched when done.
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
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        //TODO: & and | types.
        TypeValue {
            r#type: Box::new(Expression::from((
                node,
                from_simple_type(node, code_bytes),
                code_bytes,
            ))),
            ..Default::default()
        }
    }
}
impl From<&str> for TypeValue {
    fn from(name: &str) -> Self {
        TypeValue {
            r#type: Box::new(ExpressionInner::from(name).into()),
            ..Default::default()
        }
    }
}
impl From<ExpressionInner> for TypeValue {
    fn from(value: ExpressionInner) -> Self {
        TypeValue {
            r#type: Box::new(value.into()),
            ..Default::default()
        }
    }
}
impl From<Box<ExpressionInner>> for TypeValue {
    fn from(value: Box<ExpressionInner>) -> Self {
        TypeValue {
            r#type: Box::new(value.into()),
            ..Default::default()
        }
    }
}

impl Default for TypeDefinition {
    fn default() -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: "any".to_string(),
            equal_sign: None,
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

        let prefix = self
            .export_keyword
            .as_ref()
            .map_or_else(|| "".to_string(), |export| export.get_raw_value());

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
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        if node.kind() != "typeDefinition" {
            return None;
        }

        Some(vec![TypeDefinition::from((node, code_bytes, true))])
    }
}

impl From<(Node<'_>, &[u8], bool)> for TypeDefinition {
    fn from((node, code_bytes, is_definition): (Node, &[u8], bool)) -> Self {
        if is_definition {
            TypeDefinition {
                export_keyword: node
                    .child_by_field_name("export")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_keyword: node
                    .child_by_field_name("typeKeyword")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_name: node
                    .child_by_field_name("typeName")
                    .unwrap()
                    .utf8_text(code_bytes)
                    .unwrap()
                    .to_string(),
                equal_sign: node
                    .child_by_field_name("equal")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_value: Box::new(TypeValue::from((
                    node.child_by_field_name("type").unwrap(),
                    code_bytes,
                ))),
            }
        } else {
            TypeDefinition {
                export_keyword: None,
                type_keyword: None,
                type_name: "".to_string(),
                equal_sign: None,
                type_value: Box::new(TypeValue::from((node, code_bytes))),
            }
        }
    }
}

impl From<&str> for TypeDefinition {
    fn from(type_name: &str) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: type_name.to_string(),
            equal_sign: None,
            type_value: Box::new(TypeValue::from(type_name)),
        }
    }
}

impl From<ExpressionInner> for TypeDefinition {
    fn from(value: ExpressionInner) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: "".to_string(),
            equal_sign: None,
            type_value: Box::new(TypeValue::from(value)),
        }
    }
}

impl From<Box<ExpressionInner>> for TypeDefinition {
    fn from(value: Box<ExpressionInner>) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: "".to_string(),
            equal_sign: None,
            type_value: Box::from(TypeValue::from(value)),
        }
    }
}
