//! Implements helper traits for type values

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    bad_range,
    prelude::{
        Expression, FromNode, FromNodeWithArgs, HasRange, List, ListItem, Range, StringLiteral,
        Table, TableField, TableFieldValue, TableKey, Token, TypeValue,
    },
    unhandled_kind,
    utils::get_range_from_boundaries,
};

use super::helper_functions::build_function_type;

impl FromNode for TypeValue {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        match node.kind() {
            "name" => {
                let parent_node = node.parent()?;

                if let Some(opening_arrows) = parent_node.child_by_field_name("opening_arrows") {
                    Some(Self::Generic {
                        base: Token::from_node(node, code_bytes)?,
                        right_arrows: Token::from_node(opening_arrows, code_bytes)?,
                        generics: List::from_iter(
                            parent_node
                                .children_by_field_name("typeparam", &mut parent_node.walk()),
                            parent_node,
                            "typeParamSeparator",
                            code_bytes,
                            |_, node| {
                                let kind = node.kind();
                                match kind {
                                    "typeparam" => {
                                        Self::from_node(node.child(0)?, code_bytes).map(Arc::new)
                                    }
                                    "typepack" => Self::from_node(node, code_bytes).map(Arc::new),
                                    _ => unhandled_kind!(kind, "TypeValue"),
                                }
                            },
                        ),
                        left_arrows: Token::from_node(
                            parent_node.child_by_field_name("closing_arrows")?,
                            code_bytes,
                        )?,
                    })
                } else {
                    Some(TypeValue::Basic(Token::from_node(node, code_bytes)?))
                }
            }
            "namedtype" => {
                if let Some(module) = node.child_by_field_name("module") {
                    Some(Self::Module {
                        module: Token::from_node(module, code_bytes)?,
                        dot: Token::from_node(node.child_by_field_name("dot")?, code_bytes)?,
                        type_value: Arc::new(Self::from_node(
                            node.child_by_field_name("nameWithGenerics")?,
                            code_bytes,
                        )?),
                    })
                } else {
                    Self::from_node(node.child_by_field_name("nameWithGenerics")?, code_bytes)
                }
            }
            "wraptype" => Some(Self::Wrap {
                opening_parenthesis: Token::from_node(node.child(0)?, code_bytes)?,
                r#type: Self::from_node(node.child(1)?, code_bytes).map(Arc::new)?,
                closing_parenthesis: Token::from_node(node.child(2)?, code_bytes)?,
            }),
            "typeof" => Some(Self::Typeof {
                typeof_token: Token::from_node(node.child(0)?, code_bytes)?,
                opening_parenthesis: Token::from_node(node.child(1)?, code_bytes)?,
                inner: Expression::from_node(node.child(2)?, code_bytes).map(Arc::new)?,
                closing_parenthesis: Token::from_node(node.child(3)?, code_bytes)?,
            }),
            "functionType" => build_function_type(node, code_bytes),
            "tableType" => Some(Self::Table(Table::from_node(node, code_bytes, ((), ()))?)),
            "singleton" => Some(Self::Basic(Token::from_node(node, code_bytes)?)),
            "bintype" => {
                let operator = Token::from_node(node.child_by_field_name("op")?, code_bytes)?;

                let left = Self::from_node(node.child_by_field_name("arg0")?, code_bytes)?;
                let right = Self::from_node(node.child_by_field_name("arg1")?, code_bytes)?;

                if operator.word == "&" {
                    Some(Self::Intersection {
                        left: Arc::new(left),
                        ampersand: operator,
                        right: Arc::new(right),
                    })
                } else {
                    Some(Self::Union {
                        left: Arc::new(left),
                        pipe: operator,
                        right: Arc::new(right),
                    })
                }
            }
            "untype" => Some(Self::Optional {
                base: Self::from_node(node.child_by_field_name("arg")?, code_bytes)
                    .map(Arc::new)?,
                question_mark: Token::from_node(node.child_by_field_name("op")?, code_bytes)?,
            }),
            "typepack" => {
                let pack = node.child(0)?;
                match pack.kind() {
                    "(" => {
                        let opening_parenthesis = Token::from_node(
                            node.child_by_field_name("opening_parenthesis")?,
                            code_bytes,
                        )?;
                        let closing_parenthesis = Token::from_node(
                            node.child_by_field_name("closing_parenthesis")?,
                            code_bytes,
                        )?;

                        let mut types = List::from_iter(
                            node.children_by_field_name("type", &mut node.walk()),
                            node,
                            "separator",
                            code_bytes,
                            |_, node| Self::from_node(node, code_bytes).map(Arc::new),
                        );

                        if let Some(typepack) = node.child_by_field_name("variadic") {
                            types.push(ListItem::NonTrailing(Arc::new(Self::from_node(
                                typepack, code_bytes,
                            )?)))
                        }

                        Some(Self::Tuple {
                            opening_parenthesis,
                            types,
                            closing_parenthesis,
                        })
                    }
                    "variadic" | "genpack" => Self::from_node(node.child(0)?, code_bytes),
                    _ => unreachable!(),
                }
            }
            "variadic" => Some(Self::Variadic {
                ellipsis: Token::from_node(node.child(0)?, code_bytes)?,
                type_info: Self::from_node(node.child(1)?, code_bytes).map(Arc::new)?,
            }),
            "genpack" => Some(Self::GenericPack {
                name: Token::from_node(node.child(0)?, code_bytes)?,
                ellipsis: Token::from_node(node.child(1)?, code_bytes)?,
            }),
            _ => panic!("Reached unhandled type. {}", node.to_sexp()),
        }
    }
}

impl HasRange for TypeValue {
    fn get_range(&self) -> Range {
        match self {
            Self::ERROR => Range::default(),
            Self::Basic(value) | Self::String(StringLiteral(value)) | Self::Boolean(value) => {
                value.get_range()
            }
            Self::Wrap {
                opening_parenthesis,
                r#type: _,
                closing_parenthesis,
            } => get_range_from_boundaries(
                opening_parenthesis.get_range(),
                closing_parenthesis.get_range(),
            ),
            Self::Function {
                generics,
                opening_parenthesis,
                parameters: _,
                closing_parenthesis: _,
                arrow: _,
                return_type,
            } => get_range_from_boundaries(
                generics.as_ref().map_or_else(
                    || opening_parenthesis.get_range(),
                    |generics| generics.get_range(),
                ),
                return_type.get_range(),
            ),
            Self::Generic {
                base,
                right_arrows: _,
                generics: _,
                left_arrows,
            } => get_range_from_boundaries(base.get_range(), left_arrows.get_range()),
            Self::GenericPack { name, ellipsis } => {
                get_range_from_boundaries(name.get_range(), ellipsis.get_range())
            }
            Self::Intersection {
                left,
                ampersand: _,
                right,
            } => get_range_from_boundaries(left.get_range(), right.get_range()),
            Self::Union {
                left,
                pipe: _,
                right,
            } => get_range_from_boundaries(left.get_range(), right.get_range()),
            Self::Module {
                module,
                dot: _,
                type_value,
            } => get_range_from_boundaries(module.get_range(), type_value.get_range()),
            Self::Optional {
                base,
                question_mark,
            } => get_range_from_boundaries(base.get_range(), question_mark.get_range()),
            Self::Table(table) => table.get_range(),
            Self::Typeof {
                typeof_token,
                opening_parenthesis: _,
                inner: _,
                closing_parenthesis,
            } => {
                get_range_from_boundaries(typeof_token.get_range(), closing_parenthesis.get_range())
            }
            Self::Tuple {
                opening_parenthesis,
                types: _,
                closing_parenthesis,
            } => get_range_from_boundaries(
                opening_parenthesis.get_range(),
                closing_parenthesis.get_range(),
            ),
            Self::Variadic {
                ellipsis,
                type_info,
            } => get_range_from_boundaries(ellipsis.get_range(), type_info.get_range()),
            Self::VariadicPack { ellipsis, name } => {
                get_range_from_boundaries(ellipsis.get_range(), name.get_range())
            }
        }
    }
}

impl HasRange for Table {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.opening_brackets.get_range(),
            self.closing_brackets.get_range(),
        )
    }
}

impl HasRange for TableKey {
    fn get_range(&self) -> Range {
        match self {
            Self::ERROR => bad_range!("TableKey"),
            Self::String(value) => value.get_range(),
            Self::Expression {
                open_square_brackets,
                close_square_brackets,
                ..
            }
            | Self::Type {
                open_square_brackets,
                close_square_brackets,
                ..
            } => get_range_from_boundaries(
                open_square_brackets.get_range(),
                close_square_brackets.get_range(),
            ),
            Self::UndefinedNumber(_) | Self::UndefinedString(_) => Range::default(),
        }
    }
}
impl HasRange for TableFieldValue {
    fn get_range(&self) -> Range {
        match self {
            Self::ERROR => bad_range!("TableFieldValue"),
            Self::Expression(value) => value.get_range(),
            Self::Type(value) => value.get_range(),
        }
    }
}
impl HasRange for TableField {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.key.get_range(), self.value.get_range())
    }
}
