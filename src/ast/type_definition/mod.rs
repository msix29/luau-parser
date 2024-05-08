//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! [`type definitions`](TypeDefinition) from [`nodes`](Node), to implementing helper
//! traits for both [`type definitions`](TypeDefinition) and [`type values`](TypeValue).
//!

pub(crate) mod helper_functions;
mod type_value;
mod type_value_conversion;

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{
        GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, HasRange, List, LuauStatement, Range, Token, TypeDefinition,
        TypeValue,
    },
    utils::get_range_from_boundaries,
};

impl LuauStatement for TypeDefinition {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "typeDefinition" {
            return None;
        }

        Some(Self::from((node, code_bytes, true)))
    }
}

impl From<(Node<'_>, &[u8], bool)> for TypeDefinition {
    fn from((node, code_bytes, is_definition): (Node, &[u8], bool)) -> Self {
        if is_definition {
            let generics = node.child_by_field_name("generics").map(|generics_node| {
                let mut generics = List::from_iter(
                    generics_node
                        .children_by_field_name("generic_with_default", &mut generics_node.walk()),
                    generics_node,
                    "generic_with_default_separator",
                    code_bytes,
                    |_, child| GenericDeclarationParameter {
                        parameter: GenericParameterInfo::Name(Token::from((
                            child.child(0).unwrap().child(0).unwrap(),
                            code_bytes,
                        ))),
                        default: child.child(1).map(|equal| {
                            let genpack = child.child(2).unwrap();

                            GenericParameterInfoDefault::Name {
                                equal_sign: Token::from((equal, code_bytes)),
                                name: Token::from((genpack.child(0).unwrap(), code_bytes)),
                            }
                        }),
                    },
                );
                generics.extend_from_slice(&List::from_iter(
                    generics_node.children_by_field_name(
                        "generic_pack_with_default",
                        &mut generics_node.walk(),
                    ),
                    generics_node,
                    "generic_pack_with_default_separator",
                    code_bytes,
                    |_, child| {
                        let generic_pack = child.child(0).unwrap();
                        GenericDeclarationParameter {
                            parameter: GenericParameterInfo::Pack {
                                name: Token::from((generic_pack.child(0).unwrap(), code_bytes)),
                                ellipsis: Token::from((generic_pack.child(1).unwrap(), code_bytes)),
                            },
                            default: child.child(1).map(|equal| {
                                let genpack = child.child(2).unwrap();

                                GenericParameterInfoDefault::Pack {
                                    equal_sign: Token::from((equal, code_bytes)),
                                    r#type: TypeValue::from((genpack, code_bytes)),
                                }
                            }),
                        }
                    },
                ));

                GenericDeclaration {
                    opening_arrow: Token::from((
                        node.child_by_field_name("opening_arrow").unwrap(),
                        code_bytes,
                    )),
                    generics,
                    closing_arrow: Token::from((
                        node.child_by_field_name("closing_arrow").unwrap(),
                        code_bytes,
                    )),
                }
            });

            Self {
                export_keyword: node
                    .child_by_field_name("export")
                    .map(|node| Token::from((node, code_bytes))),
                type_keyword: node
                    .child_by_field_name("typeKeyword")
                    .map(|node| Token::from((node, code_bytes))),
                generics,
                type_name: Token::from((node.child_by_field_name("typeName").unwrap(), code_bytes)),
                equal_sign: node
                    .child_by_field_name("equal")
                    .map(|node| Token::from((node, code_bytes))),
                type_value: Arc::new(TypeValue::from((
                    node.child_by_field_name("type").unwrap(),
                    code_bytes,
                ))),
                #[cfg(feature = "lsp-ready")]
                references: Vec::new(),
            }
        } else {
            Self {
                export_keyword: None,
                type_keyword: None,
                type_name: Token::default(),
                generics: None,
                equal_sign: None,
                type_value: Arc::new(TypeValue::from((node, code_bytes))),
                #[cfg(feature = "lsp-ready")]
                references: Vec::new(),
            }
        }
    }
}

impl From<Token> for TypeDefinition {
    fn from(type_name: Token) -> Self {
        Self {
            export_keyword: None,
            type_keyword: None,
            type_name: type_name.clone(),
            generics: None,
            equal_sign: None,
            type_value: Arc::new(TypeValue::Basic(type_name)),
            #[cfg(feature = "lsp-ready")]
            references: Vec::new(),
        }
    }
}
impl From<TypeValue> for TypeDefinition {
    fn from(type_value: TypeValue) -> Self {
        Self {
            export_keyword: None,
            type_keyword: None,
            type_name: Token::default(),
            generics: None,
            equal_sign: None,
            type_value: Arc::new(type_value),
            #[cfg(feature = "lsp-ready")]
            references: Vec::new(),
        }
    }
}
impl HasRange for TypeDefinition {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            // `call_any!` macro just doesn't wanna work for whatever reason.
            self.export_keyword
                .as_ref()
                .or(self.type_keyword.as_ref())
                .map_or_else(|| self.type_value.get_range(), |a| a.get_range()),
            self.type_value.get_range(),
        )
    }
}

impl HasRange for GenericDeclaration {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.opening_arrow.get_range(),
            self.closing_arrow.get_range(),
        )
    }
}
impl HasRange for GenericDeclarationParameter {
    fn get_range(&self) -> Range {
        if let Some(default) = &self.default {
            get_range_from_boundaries(self.parameter.get_range(), default.get_range())
        } else {
            self.parameter.get_range()
        }
    }
}
impl HasRange for GenericParameterInfo {
    fn get_range(&self) -> Range {
        match self {
            GenericParameterInfo::Name(name) => name.get_range(),
            GenericParameterInfo::Pack { name, ellipsis } => {
                get_range_from_boundaries(name.get_range(), ellipsis.get_range())
            }
        }
    }
}
impl HasRange for GenericParameterInfoDefault {
    fn get_range(&self) -> Range {
        match self {
            GenericParameterInfoDefault::Name { equal_sign, name } => {
                get_range_from_boundaries(equal_sign.get_range(), name.get_range())
            }
            GenericParameterInfoDefault::Pack { equal_sign, r#type } => {
                get_range_from_boundaries(equal_sign.get_range(), r#type.get_range())
            }
        }
    }
}
