//! Helper functions.

use std::sync::Arc;

use tree_sitter::Node;

use crate::prelude::{
    FunctionParameter, GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo, List,
    ListItem, NormalizedName, SingleToken, TableField, TableKey, TableValue, TypeDefinition,
    TypeValue,
};

/// Get a type value from a node representing a singleton type.
pub(crate) fn from_singleton_type(node: Node, code_bytes: &[u8]) -> TypeValue {
    TypeValue::Basic(SingleToken::from((node, code_bytes)))
    // match node.kind() {
    //     "string" => TypeValue::Basic(SingleToken::from((node.utf8_text(code_bytes).unwrap(), node))),
    //     "name" => TypeValue::Basic(SingleToken::from(("<other value here>", node))),
    //     "false" => TypeValue::Basic(SingleToken::from(("false", node))),
    //     "true" => TypeValue::Basic(SingleToken::from(("true", node))),
    //     _ => TypeValue::Basic(SingleToken::from(("any", node))),
    // }
}

/// Build a table value from a node representing a table.
pub(crate) fn build_table_type(node: Node, code_bytes: &[u8]) -> TableValue {
    let opening_brackets = SingleToken::from((
        node.child_by_field_name("opening_brackets").unwrap(),
        code_bytes,
    ));
    let closing_brackets = SingleToken::from((
        node.child_by_field_name("closing_brackets").unwrap(),
        code_bytes,
    ));

    let Some(fields_list) = node
        .child_by_field_name("fields")
        .map(|node| node.child(0).unwrap())
    else {
        return TableValue {
            opening_brackets,
            fields: List::default(),
            closing_brackets,
        };
    };

    let mut table_fields = Vec::new();
    match fields_list.kind() {
        "propList" => {
            for field in fields_list.children_by_field_name("field", &mut fields_list.walk()) {
                match field.kind() {
                    "tableProperty" => {
                        table_fields.push(ListItem::NonTrailing(TableField {
                            key: Arc::new(TableKey::String(SingleToken::from((
                                field.child(0).unwrap(),
                                code_bytes,
                            )))),
                            equal_or_colon: Some(SingleToken::from((
                                field.child(1).unwrap(),
                                code_bytes,
                            ))),
                            r#type: Some(Arc::new(TypeDefinition::from((
                                field.child(2).unwrap(),
                                code_bytes,
                                false,
                            )))),
                            value: None,
                        }));
                    }
                    "tableIndexer" => {
                        table_fields.push(ListItem::NonTrailing(TableField {
                            key: Arc::new(TableKey::Type {
                                open_square_brackets: SingleToken::from((
                                    field.child(0).unwrap(),
                                    code_bytes,
                                )),
                                r#type: Arc::new(TypeDefinition::from((
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
                            r#type: Some(Arc::new(TypeDefinition::from((
                                field.child(4).unwrap(),
                                code_bytes,
                                false,
                            )))),
                            value: None,
                        }));
                    }
                    _ => (),
                }
            }
        }
        _ => table_fields.push(ListItem::NonTrailing(TableField {
            key: Arc::new(TableKey::UndefinedString("[number]".to_string())),
            equal_or_colon: None,
            r#type: Some(Arc::new(TypeDefinition::from((
                fields_list,
                code_bytes,
                false,
            )))),
            value: None,
        })),
    }

    TableValue {
        opening_brackets,
        fields: List {
            items: table_fields,
        },
        closing_brackets,
    }
}

/// Build functions parameters from a node representing a function.
pub(crate) fn build_function_parameters(
    parameters_node: Node,
    code_bytes: &[u8],
    is_type: bool,
) -> List<FunctionParameter> {
    let mut parameters = List::from_iter(
        parameters_node.children_by_field_name("parameter", &mut parameters_node.walk()),
        parameters_node,
        "separator",
        code_bytes,
        |_, parameter| {
            let normalized_name = NormalizedName::from((parameter, code_bytes));

            if let Some(r#type) = normalized_name.r#type {
                FunctionParameter {
                    name: normalized_name.name,
                    r#type: Some(r#type),
                }
            } else if !is_type {
                FunctionParameter {
                    name: normalized_name.name,
                    r#type: None,
                }
            } else {
                // Pretty sure this isn't in the spec, but if the name is missing in a type
                // definition of a function, then it's the type and the name is empty, but for
                // the sake of making it "better", we use `_`, which is globally known as a
                // placeholder.
                FunctionParameter {
                    name: SingleToken::from("_"),
                    r#type: Some(Arc::new(TypeDefinition::from((
                        parameter.child(0).unwrap(),
                        code_bytes,
                        false,
                    )))),
                }
            }
        },
    );
    if let Some(variadic) = parameters_node.child_by_field_name("variadic") {
        parameters
            .items
            .push(ListItem::NonTrailing(FunctionParameter {
                name: SingleToken::from((variadic.child(0).unwrap(), code_bytes)),
                r#type: variadic.child(2).map(|r#type| {
                    if let Some(r#type) = r#type.child_by_field_name("type") {
                        Arc::new(TypeDefinition::from((r#type, code_bytes, false)))
                    } else {
                        Arc::new(TypeDefinition::from(TypeValue::from((r#type, code_bytes))))
                    }
                }),
            }))
    }

    parameters
}

/// Build function returns from a node representing a function.
pub(crate) fn build_function_returns(node: Node, code_bytes: &[u8]) -> TypeValue {
    if let Some(return_node) = node
        .child_by_field_name("return")
        .or(node.child_by_field_name("returns"))
    {
        TypeValue::from((return_node, code_bytes))
    } else {
        TypeValue::Tuple {
            opening_parenthesis: SingleToken::from("("),
            types: List::default(),
            closing_parenthesis: SingleToken::from(")"),
        }
    }
}

/// Build a type value from a node representing a function.
pub(crate) fn build_function_type(node: Node, code_bytes: &[u8]) -> TypeValue {
    let generics = if node.child_by_field_name("generics").is_some() {
        let mut generics = List::from_iter(
            node.children_by_field_name("generic", &mut node.walk()),
            node,
            "separator",
            code_bytes,
            |_, child| GenericDeclarationParameter {
                parameter: GenericParameterInfo::Name(SingleToken::from((child, code_bytes))),
                default: None,
            },
        );
        generics.items.extend_from_slice(
            &List::from_iter(
                node.children_by_field_name("genericPack", &mut node.walk()),
                node,
                "genericPackSeparator",
                code_bytes,
                |_, child| GenericDeclarationParameter {
                    parameter: GenericParameterInfo::Pack {
                        name: SingleToken::from((child.child(0).unwrap(), code_bytes)),
                        ellipsis: SingleToken::from((child.child(1).unwrap(), code_bytes)),
                    },
                    default: None,
                },
            )
            .items,
        );

        Some(GenericDeclaration {
            left_arrow: SingleToken::from((
                node.child_by_field_name("left_arrow").unwrap(),
                code_bytes,
            )),
            generics,
            right_arrow: SingleToken::from((
                node.child_by_field_name("right_arrow").unwrap(),
                code_bytes,
            )),
        })
    } else {
        None
    };

    TypeValue::Function {
        generics,
        opening_parenthesis: SingleToken::from((
            node.child_by_field_name("opening_parenthesis").unwrap(),
            code_bytes,
        )),
        parameters: build_function_parameters(node, code_bytes, true),
        closing_parenthesis: SingleToken::from((
            node.child_by_field_name("closing_parenthesis").unwrap(),
            code_bytes,
        )),
        arrow: SingleToken::from((node.child_by_field_name("arrow").unwrap(), code_bytes)),
        return_type: Arc::new(build_function_returns(node, code_bytes)),
    }
}
