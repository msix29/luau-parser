//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! _[type definitions](TypeDefinition)_ from _[nodes](Node)_, to implementing helper
//! traits for both _[type definitions](TypeDefinition)_ and _[type values](TypeValue)_.
//!

pub(crate) mod functions;
mod type_value;

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{
        AstNode, GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, List, ListItem, SingleToken, TypeDefinition, TypeValue,
    },
    utils::get_location,
};

impl AstNode for TypeDefinition {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "typeDefinition" {
            return None;
        }

        Some(TypeDefinition::from((node, code_bytes, true)))
    }
}

impl From<(Node<'_>, &[u8], bool)> for TypeDefinition {
    fn from((node, code_bytes, is_definition): (Node, &[u8], bool)) -> Self {
        if is_definition {
            let generics = if let Some(generics_node) = node.child_by_field_name("generics") {
                let mut generics = Vec::new();
                let generic_with_default_separator = generics_node
                    .children_by_field_name(
                        "generic_with_default_separator",
                        &mut generics_node.walk(),
                    )
                    .collect::<Vec<Node>>();

                let generic_pack_with_default_separator = generics_node
                    .children_by_field_name(
                        "generic_pack_with_default_separator",
                        &mut generics_node.walk(),
                    )
                    .collect::<Vec<Node>>();

                for (i, child) in generics_node
                    .children_by_field_name("generic_with_default", &mut generics_node.walk())
                    .enumerate()
                {
                    let generic_pack = child.child(0).unwrap();
                    let generic_item = GenericDeclarationParameter {
                        parameter: GenericParameterInfo::Name(SingleToken::from((
                            generic_pack.child(0).unwrap(),
                            code_bytes,
                        ))),
                        default: child.child(1).map(|equal| {
                            let genpack = child.child(2).unwrap();

                            GenericParameterInfoDefault::Name {
                                equal_sign: SingleToken::from((equal, code_bytes)),
                                name: SingleToken::from((genpack.child(0).unwrap(), code_bytes)),
                            }
                        }),
                    };

                    if let Some(separator) = generic_with_default_separator.get(i) {
                        generics.push(ListItem::Trailing {
                            item: generic_item,
                            separator: SingleToken::from((*separator, code_bytes)),
                        })
                    } else {
                        generics.push(ListItem::NonTrailing(generic_item))
                    }
                }

                for (i, child) in generics_node
                    .children_by_field_name("generic_pack_with_default", &mut generics_node.walk())
                    .enumerate()
                {
                    let generic_pack = child.child(0).unwrap();
                    let generic_item = GenericDeclarationParameter {
                        parameter: GenericParameterInfo::Pack {
                            name: SingleToken::from((generic_pack.child(0).unwrap(), code_bytes)),
                            ellipsis: SingleToken::from((
                                generic_pack.child(1).unwrap(),
                                code_bytes,
                            )),
                        },
                        default: child.child(1).map(|equal| {
                            let genpack = child.child(2).unwrap();

                            GenericParameterInfoDefault::Pack {
                                equal_sign: SingleToken::from((equal, code_bytes)),
                                name: SingleToken::from((genpack.child(0).unwrap(), code_bytes)),
                                ellipsis: SingleToken::from((
                                    genpack.child(1).unwrap(),
                                    code_bytes,
                                )),
                            }
                        }),
                    };

                    if let Some(separator) = generic_pack_with_default_separator.get(i) {
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

            let name_node = node.child_by_field_name("typeName").unwrap();

            TypeDefinition {
                export_keyword: node
                    .child_by_field_name("export")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_keyword: node
                    .child_by_field_name("typeKeyword")
                    .map(|node| SingleToken::from((node, code_bytes))),
                generics,
                type_name: name_node.utf8_text(code_bytes).unwrap().to_string(),
                equal_sign: node
                    .child_by_field_name("equal")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_value: Arc::new(TypeValue::from((
                    node.child_by_field_name("type").unwrap(),
                    code_bytes,
                ))),
                name_location: Some(get_location(name_node)),
            }
        } else {
            TypeDefinition {
                export_keyword: None,
                type_keyword: None,
                type_name: "".to_string(),
                generics: None,
                equal_sign: None,
                type_value: Arc::new(TypeValue::from((node, code_bytes))),
                name_location: Some(get_location(node)),
            }
        }
    }
}

impl From<(&str, Node<'_>, &[u8])> for TypeDefinition {
    fn from((type_name, node, code_bytes): (&str, Node<'_>, &[u8])) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: type_name.to_string(),
            generics: None,
            equal_sign: None,
            type_value: Arc::new(TypeValue::from((type_name, node, code_bytes))),
            name_location: None,
        }
    }
}
