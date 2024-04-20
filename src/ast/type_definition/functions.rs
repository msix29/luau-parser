//! Helper functions.

use std::sync::Arc;

use tree_sitter::Node;

use crate::prelude::{
    GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo, List, ListItem,
    NormalizedName, SingleToken, Table, TableField, TableFieldValue, TableKey, TypeDefinition,
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
pub(crate) fn build_table_type(node: Node, code_bytes: &[u8]) -> Table {
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
        return Table {
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
                            value: Arc::new(TableFieldValue::Type(TypeDefinition::from((
                                field.child(2).unwrap(),
                                code_bytes,
                                false,
                            )))),
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
                            value: Arc::new(TableFieldValue::Type(TypeDefinition::from((
                                field.child(4).unwrap(),
                                code_bytes,
                                false,
                            )))),
                        }));
                    }
                    _ => (),
                }
            }
        }
        _ => table_fields.push(ListItem::NonTrailing(TableField {
            key: Arc::new(TableKey::UndefinedString("[number]".to_string())),
            equal_or_colon: None,
            value: Arc::new(TableFieldValue::Type(TypeDefinition::from((
                fields_list,
                code_bytes,
                false,
            )))),
        })),
    }

    Table {
        opening_brackets,
        fields: List {
            items: table_fields,
        },
        closing_brackets,
    }
}

/// Build functions parameters from a node representing a function.
pub(crate) fn build_function_parameters(
    node: Node,
    code_bytes: &[u8],
    is_type: bool,
) -> List<NormalizedName> {
    let mut parameters = List::from_iter(
        node.children_by_field_name("parameter", &mut node.walk()),
        node,
        "parameterSeparator",
        code_bytes,
        |_, parameter| {
            let normalized_name = NormalizedName::from((parameter, code_bytes));

            if is_type && normalized_name.r#type.is_none() {
                // Pretty sure this isn't in the spec, but if the name is missing in a type
                // definition of a function, then it's the type and the name is empty, but for
                // the sake of making it "better", we use `_`, which is globally known as a
                // placeholder.
                NormalizedName {
                    name: SingleToken::from("_"),
                    colon: None,
                    r#type: Some(Arc::new(TypeDefinition::from((
                        parameter.child(0).unwrap(),
                        code_bytes,
                        false,
                    )))),
                }
            } else {
                normalized_name
            }
        },
    );
    if let Some(variadic) = node.child_by_field_name("variadic") {
        let name = if node.kind() == "functionType" {
            NormalizedName {
                name: SingleToken::from(""),
                colon: None,
                r#type: Some(Arc::new(TypeDefinition::from((
                    variadic.child(0).unwrap(),
                    code_bytes,
                    false,
                )))),
            }
        } else {
            NormalizedName {
                name: SingleToken::from((variadic.child(0).unwrap(), code_bytes)),
                colon: variadic
                    .child(1)
                    .map(|colon| SingleToken::from((colon, code_bytes))),
                r#type: variadic.child(2).map(|r#type| {
                    if let Some(r#type) = r#type.child_by_field_name("type") {
                        Arc::new(TypeDefinition::from((r#type, code_bytes, false)))
                    } else {
                        Arc::new(TypeDefinition::from(TypeValue::from((r#type, code_bytes))))
                    }
                }),
            }
        };

        parameters.items.push(ListItem::NonTrailing(name));
    }

    parameters
}

/// Build function returns from a node representing a function.
pub(crate) fn build_function_returns(node: Node, code_bytes: &[u8]) -> Option<TypeValue> {
    node.child_by_field_name("return")
        .or(node.child_by_field_name("returns"))
        .map(|return_node| TypeValue::from((return_node, code_bytes)))
}

/// Build the generics of a function.
pub(crate) fn build_generics(node: Node, code_bytes: &[u8]) -> Option<GenericDeclaration> {
    if node.child_by_field_name("generics").is_some() {
        let mut generics = List::from_iter(
            node.children_by_field_name("generic", &mut node.walk()),
            node,
            "genericsSeparator",
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
            opening_arrow: SingleToken::from((
                node.child_by_field_name("opening_arrow").unwrap(),
                code_bytes,
            )),
            generics,
            closing_arrow: SingleToken::from((
                node.child_by_field_name("closing_arrow").unwrap(),
                code_bytes,
            )),
        })
    } else {
        None
    }
}

/// Build a type value from a node representing a function.
pub(crate) fn build_function_type(node: Node, code_bytes: &[u8]) -> TypeValue {
    TypeValue::Function {
        generics: build_generics(node, code_bytes),
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
        // Function types will always have a return.
        return_type: Arc::new(build_function_returns(node, code_bytes).unwrap()),
    }
}
