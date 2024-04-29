//! Implements helper traits for type values

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{
        Expression, HasRange, List, ListItem, Range, StringLiteral, Table, TableField,
        TableFieldValue, TableKey, Token, TypeValue,
    },
    utils::get_range_from_boundaries,
};

use super::helper_functions::{build_function_type, build_table_type, from_singleton_type};

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        if node.is_error() | node.is_missing() {
            return Self::ERROR;
        }

        match node.kind() {
            "name" => {
                let parent_node = node.parent().unwrap();

                if let Some(opening_arrows) = parent_node.child_by_field_name("opening_arrows") {
                    Self::Generic {
                        base: Token::from((node, code_bytes)),
                        right_arrows: Token::from((opening_arrows, code_bytes)),
                        generics: List::from_iter(
                            parent_node
                                .children_by_field_name("typeparam", &mut parent_node.walk()),
                            parent_node,
                            "typeParamSeparator",
                            code_bytes,
                            |_, node| match node.kind() {
                                "typeparam" => {
                                    Arc::new(Self::from((node.child(0).unwrap(), code_bytes)))
                                }
                                "typepack" => Arc::new(Self::from((node, code_bytes))),
                                _ => unreachable!("{}", node.kind()),
                            },
                        ),
                        left_arrows: Token::from((
                            parent_node.child_by_field_name("closing_arrows").unwrap(),
                            code_bytes,
                        )),
                    }
                } else {
                    from_singleton_type(node, code_bytes)
                }
            }
            "namedtype" => {
                if let Some(module) = node.child_by_field_name("module") {
                    Self::Module {
                        module: Token::from((module, code_bytes)),
                        dot: Token::from((node.child_by_field_name("dot").unwrap(), code_bytes)),
                        type_value: Arc::new(Self::from((
                            node.child_by_field_name("nameWithGenerics").unwrap(),
                            code_bytes,
                        ))),
                    }
                } else {
                    Self::from((
                        node.child_by_field_name("nameWithGenerics").unwrap(),
                        code_bytes,
                    ))
                }
            }
            "wraptype" => Self::Wrap {
                opening_parenthesis: Token::from((node.child(0).unwrap(), code_bytes)),
                r#type: Arc::new(Self::from((node.child(1).unwrap(), code_bytes))),
                closing_parenthesis: Token::from((node.child(2).unwrap(), code_bytes)),
            },
            "typeof" => Self::Typeof {
                typeof_token: Token::from((node.child(0).unwrap(), code_bytes)),
                opening_parenthesis: Token::from((node.child(1).unwrap(), code_bytes)),
                inner: Arc::new(Expression::from((node.child(2).unwrap(), code_bytes))),
                closing_parenthesis: Token::from((node.child(3).unwrap(), code_bytes)),
            },
            "functionType" => build_function_type(node, code_bytes),
            "tableType" => Self::Table(build_table_type(node, code_bytes)),
            "singleton" => from_singleton_type(node, code_bytes),
            "bintype" => {
                let operator = Token::from((node.child_by_field_name("op").unwrap(), code_bytes));

                let left = Self::from((node.child_by_field_name("arg0").unwrap(), code_bytes));
                let right = Self::from((node.child_by_field_name("arg1").unwrap(), code_bytes));

                if operator.word == "&" {
                    Self::Intersection {
                        left: Arc::new(left),
                        ampersand: operator,
                        right: Arc::new(right),
                    }
                } else {
                    Self::Union {
                        left: Arc::new(left),
                        pipe: operator,
                        right: Arc::new(right),
                    }
                }
            }
            "untype" => Self::Optional {
                base: Arc::new(Self::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
                question_mark: Token::from((node.child_by_field_name("op").unwrap(), code_bytes)),
            },
            "typepack" => {
                let pack = node.child(0).unwrap();
                match pack.kind() {
                    "(" => {
                        let opening_parenthesis = Token::from((
                            node.child_by_field_name("opening_parenthesis").unwrap(),
                            code_bytes,
                        ));
                        let closing_parenthesis = Token::from((
                            node.child_by_field_name("closing_parenthesis").unwrap(),
                            code_bytes,
                        ));

                        let mut types = List::from_iter(
                            node.children_by_field_name("type", &mut node.walk()),
                            node,
                            "separator",
                            code_bytes,
                            |_, node| Arc::new(Self::from((node, code_bytes))),
                        );

                        if let Some(typepack) = node.child_by_field_name("variadic") {
                            types.push(ListItem::NonTrailing(Arc::new(Self::from((
                                typepack, code_bytes,
                            )))))
                        }

                        Self::Tuple {
                            opening_parenthesis,
                            types,
                            closing_parenthesis,
                        }
                    }
                    "variadic" => Self::from((node.child(0).unwrap(), code_bytes)),
                    "genpack" => Self::from((node.child(0).unwrap(), code_bytes)),
                    _ => unreachable!(),
                }
            }
            "variadic" => Self::Variadic {
                ellipsis: Token::from((node.child(0).unwrap(), code_bytes)),
                type_info: Arc::new(Self::from((node.child(1).unwrap(), code_bytes))),
            },
            "genpack" => Self::GenericPack {
                name: Token::from((node.child(0).unwrap(), code_bytes)),
                ellipsis: Token::from((node.child(1).unwrap(), code_bytes)),
            },
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
                generics: _,
                opening_parenthesis,
                parameters: _,
                closing_parenthesis: _,
                arrow: _,
                return_type,
            } => get_range_from_boundaries(
                //TODO: Try generics here.
                opening_parenthesis.get_range(),
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
            TableKey::String(value) => value.get_range(),
            TableKey::Expression {
                open_square_brackets,
                expression: _,
                close_square_brackets,
            } => get_range_from_boundaries(
                open_square_brackets.get_range(),
                close_square_brackets.get_range(),
            ),
            TableKey::Type {
                open_square_brackets,
                r#type: _,
                close_square_brackets,
            } => get_range_from_boundaries(
                open_square_brackets.get_range(),
                close_square_brackets.get_range(),
            ),
            TableKey::UndefinedNumber(_) => Range::default(),
            TableKey::UndefinedString(_) => Range::default(),
        }
    }
}
impl HasRange for TableFieldValue {
    fn get_range(&self) -> Range {
        match self {
            TableFieldValue::Expression(value) => value.get_range(),
            TableFieldValue::Type(value) => value.get_range(),
        }
    }
}
impl HasRange for TableField {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.key.get_range(), self.value.get_range())
    }
}
