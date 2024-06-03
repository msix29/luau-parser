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
    bad_range,
    types::{
        FromNode, FromNodeWithArgs, GenericDeclaration, GenericDeclarationParameter,
        GenericParameterInfo, GenericParameterInfoDefault, HasRange, List, LuauStatement, Range,
        Token, TypeDefinition, TypeValue,
    },
    utils::{get_range_from_boundaries, map_option},
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

        Self::from_node(node, code_bytes, true)
    }
}

impl FromNodeWithArgs<bool> for TypeDefinition {
    fn from_node(node: Node, code_bytes: &[u8], is_definition: bool) -> Option<Self> {
        if is_definition {
            let generics = map_option(node.child_by_field_name("generics"), |generics_node| {
                let generics_node = generics_node?;
                let mut generics = List::from_iter(
                    generics_node
                        .children_by_field_name("generic_with_default", &mut generics_node.walk()),
                    generics_node,
                    "generic_with_default_separator",
                    code_bytes,
                    |_, child| {
                        Some(GenericDeclarationParameter {
                            parameter: GenericParameterInfo::Name(Token::from_node(
                                child.child(0)?.child(0)?,
                                code_bytes,
                            )?),
                            default: map_option(child.child(1), |equal| {
                                let genpack = child.child(2)?;

                                Some(GenericParameterInfoDefault::Name {
                                    equal_sign: Token::from_node(equal?, code_bytes)?,
                                    name: Token::from_node(genpack.child(0)?, code_bytes)?,
                                })
                            }),
                        })
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
                        let generic_pack = child.child(0)?;
                        Some(GenericDeclarationParameter {
                            parameter: GenericParameterInfo::Pack {
                                name: Token::from_node(generic_pack.child(0)?, code_bytes)?,
                                ellipsis: Token::from_node(generic_pack.child(1)?, code_bytes)?,
                            },
                            default: map_option(child.child(1), |equal| {
                                let genpack = child.child(2)?;

                                Some(GenericParameterInfoDefault::Pack {
                                    equal_sign: Token::from_node(equal?, code_bytes)?,
                                    r#type: TypeValue::from_node(genpack, code_bytes)?,
                                })
                            }),
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
            });

            Some(Self {
                export_keyword: map_option(node.child_by_field_name("export"), |node| {
                    Token::from_node(node?, code_bytes)
                }),
                type_keyword: map_option(node.child_by_field_name("typeKeyword"), |node| {
                    Token::from_node(node?, code_bytes)
                }),
                generics,
                type_name: Token::from_node(node.child_by_field_name("typeName")?, code_bytes)?,
                equal_sign: map_option(node.child_by_field_name("equal"), |node| {
                    Token::from_node(node?, code_bytes)
                }),
                type_value: TypeValue::from_node(node.child_by_field_name("type")?, code_bytes)
                    .map(Arc::new)?,
            })
        } else {
            Some(Self {
                export_keyword: None,
                type_keyword: None,
                type_name: Token::default(),
                generics: None,
                equal_sign: None,
                type_value: TypeValue::from_node(node, code_bytes).map(Arc::new)?,
            })
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
            GenericParameterInfo::ERROR => bad_range!("GenericParameterInfo"),
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
