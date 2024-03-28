//! Helper functions.

use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        FunctionParameter, GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo, List, ListItem, NormalizedName, SingleToken, TableField, TableKey, TableValue, TypeDefinition, TypeValue
    },
    utils::get_location,
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
            location: get_location(node),
        };
    };
    let separators = fields_list
        .children_by_field_name("sep", &mut node.walk())
        .collect::<Vec<Node>>();
    let mut table_fields: Vec<TableField> = Vec::new();
    match fields_list.kind() {
        "type" => {
            table_fields.push(TableField {
                key: Arc::new(TableKey::String("[number]".to_string())),
                equal_or_colon: None,
                r#type: Some(Arc::new(TypeDefinition::from((
                    fields_list,
                    code_bytes,
                    false,
                )))),
                value: None,
                location: get_location(node),
                key_location: None,
                value_location: get_location(node),
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
                            key: Arc::new(TableKey::String(
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
                            r#type: Some(Arc::new(TypeDefinition::from((
                                field.child(2).unwrap(),
                                code_bytes,
                                false,
                            )))),
                            value: None,
                            //TODO
                            location: get_location(node),
                            key_location: Some(get_location(field.child(0).unwrap())),
                            value_location: get_location(field.child(0).unwrap()),
                        });
                    }
                    "tableIndexer" => {
                        table_fields.push(TableField {
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
                            location: get_location(node),
                            key_location: Some(get_location(field.child(0).unwrap())),
                            value_location: get_location(field.child(0).unwrap()),
                        });
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    TableValue {
        opening_brackets,
        fields: List::default(),
        closing_brackets,
        location: get_location(node),
    }
}

/// Build functions parameters from a node representing a function.
pub(crate) fn build_function_parameters(
    parameters_node: Node,
    code_bytes: &[u8],
    is_type: bool,
) -> List<FunctionParameter> {
    let mut parameters = Vec::new();

    let separators = parameters_node
        .children_by_field_name("separators", &mut parameters_node.walk())
        .map(|node| SingleToken::from((node, code_bytes)))
        .collect::<Vec<SingleToken>>();

    for (i, parameter) in parameters_node
        .children_by_field_name("parameter", &mut parameters_node.walk())
        .enumerate()
    {
        let normalized_name = NormalizedName::from((parameter, code_bytes));

        let parameter = if let Some(r#type) = normalized_name.r#type {
            FunctionParameter {
                name: normalized_name.name,
                is_variadic: false,
                r#type,
                location: get_location(parameter),
            }
        } else if !is_type {
            FunctionParameter {
                name: normalized_name.name,
                is_variadic: false,
                r#type: Arc::new(TypeDefinition::from(("any", parameter, code_bytes))),
                location: get_location(parameter),
            }
        } else {
            // Pretty sure this isn't in the spec, but if the name is missing in a type
            // definition of a function, then it's the type and the name is empty, but for
            // the sake of making it "better", we use `_`, which is globally known as a
            // placeholder.
            FunctionParameter {
                name: "_".to_string(),
                is_variadic: false,
                r#type: Arc::new(TypeDefinition::from((
                    normalized_name.name.as_str(),
                    parameter,
                    code_bytes,
                ))),
                location: get_location(parameter),
            }
        };

        parameters.push(if let Some(separator) = separators.get(i) {
            ListItem::Trailing {
                item: parameter,
                separator: separator.clone(),
            }
        } else {
            ListItem::NonTrailing(parameter)
        })
    }

    List { items: parameters }
}

/// Build function returns from a node representing a function.
pub(crate) fn build_function_returns(node: Node, code_bytes: &[u8]) -> TypeValue {
    TypeValue::from((
        node.child_by_field_name("return")
            .unwrap_or_else(|| node.child_by_field_name("returns").unwrap()),
        code_bytes,
    ))
}

/// Build a type value from a node representing a function.
pub(crate) fn build_function_type(node: Node, code_bytes: &[u8]) -> TypeValue {
    let generics = if node.child_by_field_name("generics").is_some() {
        let mut generics = Vec::new();
        let generic_separators = node
            .children_by_field_name("separator", &mut node.walk())
            .collect::<Vec<Node>>();

        let generic_pack_separators = node
            .children_by_field_name("genericPackSeparator", &mut node.walk())
            .collect::<Vec<Node>>();

        for (i, child) in node
            .children_by_field_name("generic", &mut node.walk())
            .enumerate()
        {
            let generic_item = GenericDeclarationParameter {
                parameter: GenericParameterInfo::Name(SingleToken::from((child, code_bytes))),
                default: None,
            };

            if let Some(separator) = generic_separators.get(i) {
                generics.push(ListItem::Trailing {
                    item: generic_item,
                    separator: SingleToken::from((*separator, code_bytes)),
                })
            } else {
                generics.push(ListItem::NonTrailing(generic_item))
            }
        }

        for (i, child) in node
            .children_by_field_name("genericPack", &mut node.walk())
            .enumerate()
        {
            let generic_item = GenericDeclarationParameter {
                parameter: GenericParameterInfo::Pack {
                    name: SingleToken::from((child.child(0).unwrap(), code_bytes)),
                    ellipsis: SingleToken::from((child.child(1).unwrap(), code_bytes)),
                },
                default: None,
            };

            if let Some(separator) = generic_pack_separators.get(i) {
                generics.push(ListItem::Trailing {
                    item: generic_item,
                    separator: SingleToken::from((*separator, code_bytes)),
                })
            } else {
                generics.push(ListItem::NonTrailing(generic_item))
            }
        }

        Some(GenericDeclaration {
            left_arrow: SingleToken::from((
                node.child_by_field_name("left_arrow").unwrap(),
                code_bytes,
            )),
            generics: List { items: generics },
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
