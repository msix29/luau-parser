//! Helper functions.

use smol_str::SmolStr;
use std::sync::Arc;
use tree_sitter::Node;

#[cfg(feature = "references")]
use crate::types::References;
use crate::{
    types::{
        FromNode, FromNodeWithArgs, GenericDeclaration, GenericDeclarationParameter,
        GenericParameterInfo, List, ListItem, NormalizedName, StringLiteral, Table, TableField,
        TableFieldValue, TableKey, Token, TypeValue,
    },
    utils::map_option,
};

impl FromNodeWithArgs<((), ())> for Table {
    /// Creates a [`Table`] type from the passed node. for [`Table`] expression, pass the
    /// 3rd argument as `()` only.
    fn from_node(node: Node, code_bytes: &[u8], _: ((), ())) -> Option<Self> {
        let opening_brackets =
            Token::from_node(node.child_by_field_name("opening_brackets")?, code_bytes)?;
        let closing_brackets =
            Token::from_node(node.child_by_field_name("closing_brackets")?, code_bytes)?;

        let Some(fields_list) =
            map_option(node.child_by_field_name("fields"), |node| node?.child(0))
        else {
            return Some(Table {
                opening_brackets,
                fields: List::default(),
                closing_brackets,
            });
        };

        let mut table_fields = Vec::new();
        match fields_list.kind() {
            "propList" => {
                for field in fields_list.children_by_field_name("field", &mut fields_list.walk()) {
                    match field.kind() {
                        "tableProperty" => {
                            table_fields.push(ListItem::NonTrailing(TableField {
                                key: Arc::new(TableKey::String(StringLiteral::from_node(
                                    field.child(0)?,
                                    code_bytes,
                                )?)),
                                equal_or_colon: Some(Token::from_node(
                                    field.child(1)?,
                                    code_bytes,
                                )?),
                                value: Arc::new(TableFieldValue::Type(TypeValue::from_node(
                                    field.child(2).unwrap(),
                                    code_bytes,
                                )?)),
                            }));
                        }
                        "tableIndexer" => {
                            table_fields.push(ListItem::NonTrailing(TableField {
                                key: Arc::new(TableKey::Type {
                                    open_square_brackets: Token::from_node(
                                        field.child(0)?,
                                        code_bytes,
                                    )?,
                                    r#type: Arc::new(TypeValue::from_node(
                                        field.child(1).unwrap(),
                                        code_bytes,
                                    )?),
                                    close_square_brackets: Token::from_node(
                                        field.child(2)?,
                                        code_bytes,
                                    )?,
                                }),
                                equal_or_colon: Token::from_node(field.child(3)?, code_bytes),
                                value: Arc::new(TableFieldValue::Type(TypeValue::from_node(
                                    field.child(4).unwrap(),
                                    code_bytes,
                                )?)),
                            }));
                        }
                        _ => (),
                    }
                }
            }
            _ => table_fields.push(ListItem::NonTrailing(TableField {
                key: Arc::new(TableKey::UndefinedString(SmolStr::new("[number]"))),
                equal_or_colon: None,
                value: Arc::new(TableFieldValue::Type(TypeValue::from_node(
                    fields_list,
                    code_bytes,
                )?)),
            })),
        }

        Some(Table {
            opening_brackets,
            fields: List {
                items: table_fields,
            },
            closing_brackets,
        })
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
            let normalized_name = NormalizedName::from_node(parameter, code_bytes)?;

            if is_type && normalized_name.r#type.is_none() {
                // Pretty sure this isn't in the spec, but if the name is missing in a type
                // definition of a function, then it's the type and the name is empty, but for
                // the sake of making it "better", we use `_`, which is globally known as a
                // placeholder.
                Some(NormalizedName {
                    name: Token::from("_"),
                    colon: None,
                    r#type: TypeValue::from_node(parameter.child(0)?, code_bytes).map(Arc::new),
                    #[cfg(feature = "references")]
                    references: References::new(),
                })
            } else {
                Some(normalized_name)
            }
        },
    );
    if let Some(variadic) = map_option(node.child_by_field_name("variadic"), |variadic| {
        let variadic = variadic?;
        let name = if node.kind() == "functionType" {
            NormalizedName {
                name: Token::from(""),
                colon: None,
                r#type: TypeValue::from_node(variadic.child(0)?, code_bytes).map(Arc::new),
                #[cfg(feature = "references")]
                references: References::new(),
            }
        } else {
            NormalizedName {
                name: Token::from_node(variadic.child(0)?, code_bytes)?,
                colon: map_option(variadic.child(1), |colon| {
                    Token::from_node(colon?, code_bytes)
                }),
                r#type: map_option(variadic.child(2), |r#type| {
                    let r#type = r#type?;
                    if let Some(r#type) = r#type.child_by_field_name("type") {
                        TypeValue::from_node(r#type, code_bytes).map(Arc::new)
                    } else {
                        TypeValue::from_node(r#type, code_bytes).map(Arc::new)
                    }
                }),
                #[cfg(feature = "references")]
                references: References::new(),
            }
        };

        Some(ListItem::NonTrailing(name))
    }) {
        parameters.push(variadic);
    };

    parameters
}

/// Build function returns from a node representing a function.
pub(crate) fn build_function_returns(node: Node, code_bytes: &[u8]) -> Option<Arc<TypeValue>> {
    let node = node
        .child_by_field_name("return")
        .or(node.child_by_field_name("returns"));

    map_option(node, |return_node| {
        TypeValue::from_node(return_node?, code_bytes).map(Arc::new)
    })
}

/// Build the generics of a function.
pub(crate) fn build_generics(node: Node, code_bytes: &[u8]) -> Option<GenericDeclaration> {
    if node.child_by_field_name("generics").is_some() {
        let mut generics = List::from_iter(
            node.children_by_field_name("generic", &mut node.walk()),
            node,
            "genericsSeparator",
            code_bytes,
            |_, child| {
                Some(GenericDeclarationParameter {
                    parameter: GenericParameterInfo::Name(Token::from_node(child, code_bytes)?),
                    default: None,
                })
            },
        );
        generics.extend_from_slice(&List::from_iter(
            node.children_by_field_name("genericPack", &mut node.walk()),
            node,
            "genericPackSeparator",
            code_bytes,
            |_, child| {
                Some(GenericDeclarationParameter {
                    parameter: GenericParameterInfo::Pack {
                        name: Token::from_node(child.child(0)?, code_bytes)?,
                        ellipsis: Token::from_node(child.child(1)?, code_bytes)?,
                    },
                    default: None,
                })
            },
        ));

        Some(GenericDeclaration {
            opening_arrow: Token::from_node(
                node.child_by_field_name("opening_arrow")?,
                code_bytes,
            )?,
            generics,
            closing_arrow: Token::from_node(
                node.child_by_field_name("closing_arrow")?,
                code_bytes,
            )?,
        })
    } else {
        None
    }
}

/// Build a type value from a node representing a function.
#[inline]
pub(crate) fn build_function_type(node: Node, code_bytes: &[u8]) -> Option<TypeValue> {
    Some(TypeValue::Function {
        generics: build_generics(node, code_bytes),
        opening_parenthesis: Token::from_node(
            node.child_by_field_name("opening_parenthesis")?,
            code_bytes,
        )?,
        parameters: build_function_parameters(node, code_bytes, true),
        closing_parenthesis: Token::from_node(
            node.child_by_field_name("closing_parenthesis")?,
            code_bytes,
        )?,
        arrow: Token::from_node(node.child_by_field_name("arrow").unwrap(), code_bytes)?,
        // Function types will always have a return.
        return_type: build_function_returns(node, code_bytes).unwrap(),
    })
}
