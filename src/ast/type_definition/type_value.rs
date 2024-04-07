//! Implements helper traits for type values

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{
        ConversionError, ElseIfExpression, Expression, ExpressionInner, HasLocation, List, ListItem, Location, SingleToken, TableField, TableFieldValue, TableKey, TableValue, TypeValue
    },
    utils::get_location_from_boundaries,
};

use super::functions::{build_function_type, build_table_type, from_singleton_type};

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "namedtype" => {
                if let Some(module) = node.child_by_field_name("module") {
                    TypeValue::Module {
                        module: SingleToken::from((module, code_bytes)),
                        dot: SingleToken::from((
                            node.child_by_field_name("dot").unwrap(),
                            code_bytes,
                        )),
                        type_info: SingleToken::from((
                            node.child_by_field_name("name").unwrap(),
                            code_bytes,
                        )),
                    }
                } else {
                    from_singleton_type(node, code_bytes)
                }
            }
            "wraptype" => TypeValue::Wrap {
                opening_parenthesis: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                r#type: Arc::new(TypeValue::from((node.child(1).unwrap(), code_bytes))),
                closing_parenthesis: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            },
            "typeof" => TypeValue::Typeof {
                typeof_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                opening_parenthesis: SingleToken::from((node.child(1).unwrap(), code_bytes)),
                inner: Arc::new(Expression::from((node.child(2).unwrap(), code_bytes))),
                closing_parenthesis: SingleToken::from((node.child(3).unwrap(), code_bytes)),
            },
            "functionType" => build_function_type(node, code_bytes),
            "tableType" => TypeValue::Table(build_table_type(node, code_bytes)),
            "singleton" => from_singleton_type(node, code_bytes),
            "bintype" => {
                let operator =
                    SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes));

                let left = TypeValue::from((node.child_by_field_name("arg0").unwrap(), code_bytes));
                let right =
                    TypeValue::from((node.child_by_field_name("arg1").unwrap(), code_bytes));

                if operator.word == "&" {
                    TypeValue::Intersection {
                        left: Arc::new(left),
                        ampersand: operator,
                        right: Arc::new(right),
                    }
                } else {
                    TypeValue::Union {
                        left: Arc::new(left),
                        pipe: operator,
                        right: Arc::new(right),
                    }
                }
            }
            "untype" => TypeValue::Optional {
                base: Arc::new(TypeValue::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
                question_mark: SingleToken::from((
                    node.child_by_field_name("op").unwrap(),
                    code_bytes,
                )),
            },
            "typepack" => {
                let pack = node.child(0).unwrap();
                match pack.kind() {
                    "(" => {
                        let opening_parenthesis = SingleToken::from((
                            node.child_by_field_name("opening_parenthesis").unwrap(),
                            code_bytes,
                        ));
                        let closing_parenthesis = SingleToken::from((
                            node.child_by_field_name("closing_parenthesis").unwrap(),
                            code_bytes,
                        ));

                        let mut types = List::from_iter(
                            node.children_by_field_name("type", &mut node.walk()),
                            node,
                            "separator",
                            code_bytes,
                            |_, node| TypeValue::from((node, code_bytes)),
                        );

                        if let Some(typepack) = node.child_by_field_name("variadic") {
                            types.items.push(ListItem::NonTrailing(TypeValue::from((
                                typepack, code_bytes,
                            ))))
                        }

                        TypeValue::Tuple {
                            opening_parenthesis,
                            types,
                            closing_parenthesis,
                        }
                    }
                    "variadic" => TypeValue::Variadic {
                        ellipsis: SingleToken::from((pack.child(1).unwrap(), code_bytes)),
                        type_info: Arc::new(TypeValue::from((pack.child(0).unwrap(), code_bytes))),
                    },
                    "genpack" => TypeValue::GenericPack {
                        name: SingleToken::from((pack.child(1).unwrap(), code_bytes)),
                        ellipsis: SingleToken::from((pack.child(0).unwrap(), code_bytes)),
                    },
                    _ => unreachable!(),
                }
            }
            "variadic" => TypeValue::Variadic {
                ellipsis: SingleToken::from((node.child(1).unwrap(), code_bytes)),
                type_info: Arc::new(TypeValue::from((node.child(0).unwrap(), code_bytes))),
            },
            "genpack" => TypeValue::GenericPack {
                name: SingleToken::from((node.child(1).unwrap(), code_bytes)),
                ellipsis: SingleToken::from((node.child(0).unwrap(), code_bytes)),
            },
            _ => panic!("Reached unhandled type. {}", node.to_sexp()),
        }
    }
}

impl HasLocation for TypeValue {
    fn get_location(&self) -> Location {
        match self {
            TypeValue::Basic(value) | TypeValue::String(value) | TypeValue::Boolean(value) => {
                value.location
            }
            TypeValue::Wrap {
                opening_parenthesis,
                r#type: _,
                closing_parenthesis,
            } => get_location_from_boundaries(
                opening_parenthesis.get_location(),
                closing_parenthesis.get_location(),
            ),
            TypeValue::Function {
                generics: _,
                opening_parenthesis,
                parameters: _,
                closing_parenthesis: _,
                arrow: _,
                return_type,
            } => get_location_from_boundaries(
                //TODO: Try generics here.
                opening_parenthesis.get_location(),
                return_type.get_location(),
            ),
            TypeValue::Generic {
                base,
                right_arrows: _,
                generics: _,
                left_arrows,
            } => get_location_from_boundaries(base.get_location(), left_arrows.get_location()),
            TypeValue::GenericPack { name, ellipsis } => {
                get_location_from_boundaries(name.get_location(), ellipsis.get_location())
            }
            TypeValue::Intersection {
                left,
                ampersand: _,
                right,
            } => get_location_from_boundaries(left.get_location(), right.get_location()),
            TypeValue::Union {
                left,
                pipe: _,
                right,
            } => get_location_from_boundaries(left.get_location(), right.get_location()),
            TypeValue::Module {
                module,
                dot: _,
                type_info,
            } => get_location_from_boundaries(module.get_location(), type_info.get_location()),
            TypeValue::Optional {
                base,
                question_mark,
            } => get_location_from_boundaries(base.get_location(), question_mark.get_location()),
            TypeValue::Table(table) => table.get_location(),
            TypeValue::Typeof {
                typeof_token,
                opening_parenthesis: _,
                inner: _,
                closing_parenthesis,
            } => get_location_from_boundaries(
                typeof_token.get_location(),
                closing_parenthesis.get_location(),
            ),
            TypeValue::Tuple {
                opening_parenthesis,
                types: _,
                closing_parenthesis,
            } => get_location_from_boundaries(
                opening_parenthesis.get_location(),
                closing_parenthesis.get_location(),
            ),
            TypeValue::Variadic {
                ellipsis,
                type_info,
            } => get_location_from_boundaries(ellipsis.get_location(), type_info.get_location()),
            TypeValue::VariadicPack { ellipsis, name } => {
                get_location_from_boundaries(ellipsis.get_location(), name.get_location())
            }
        }
    }
}

impl HasLocation for TableValue {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(
            self.opening_brackets.get_location(),
            self.closing_brackets.get_location(),
        )
    }
}

impl HasLocation for TableKey {
    fn get_location(&self) -> Location {
        match self {
            TableKey::String(value) => value.get_location(),
            TableKey::Expression {
                open_square_brackets,
                expression: _,
                close_square_brackets,
            } => get_location_from_boundaries(
                open_square_brackets.get_location(),
                close_square_brackets.get_location(),
            ),
            TableKey::Type {
                open_square_brackets,
                r#type: _,
                close_square_brackets,
            } => get_location_from_boundaries(
                open_square_brackets.get_location(),
                close_square_brackets.get_location(),
            ),
            TableKey::UndefinedNumber(_) => Location::default(),
            TableKey::UndefinedString(_) => Location::default(),
        }
    }
}
impl HasLocation for TableFieldValue {
    fn get_location(&self) -> Location {
        match self {
            TableFieldValue::Expression(value) => value.get_location(),
            TableFieldValue::Type(value) => value.get_location(),
        }
    }
}
impl HasLocation for TableField {
    fn get_location(&self) -> Location {
        get_location_from_boundaries(self.key.get_location(), self.value.get_location())
    }
}

/// Recursively turn else if expressions to a type value.
fn else_if_to_type(
    else_if_expressions: Vec<ElseIfExpression>,
    i: usize,
) -> Result<TypeValue, ConversionError> {
    if else_if_expressions.get(i + 1).is_some() {
        Ok(TypeValue::Union {
            left: Arc::new(TypeValue::try_from(
                (*else_if_expressions.get(i).unwrap().expression.inner).clone(),
            )?),
            pipe: SingleToken::from(" | "),
            right: Arc::new(else_if_to_type(else_if_expressions, i + 1)?),
        })
    } else {
        TypeValue::try_from((*else_if_expressions.get(i).unwrap().expression.inner).clone())
    }
}

impl TryFrom<ExpressionInner> for TypeValue {
    type Error = ConversionError;

    fn try_from(value: ExpressionInner) -> Result<Self, Self::Error> {
        match value {
            ExpressionInner::Nil(value) => Ok(Self::Basic(value)),
            ExpressionInner::Boolean(word) => Ok(Self::Boolean(word)),
            ExpressionInner::Number(_) => Ok(Self::Basic(SingleToken::new("number"))),
            ExpressionInner::String(value) => Ok(Self::String(value)),
            ExpressionInner::Function {
                generics,
                opening_parenthesis,
                closing_parenthesis,
                parameters,
                returns,
                ..
            } => Ok(Self::Function {
                generics,
                opening_parenthesis,
                parameters,
                closing_parenthesis,
                arrow: SingleToken::new("->"),
                return_type: returns,
            }),
            ExpressionInner::FunctionCall(value) => Err(ConversionError::FunctionCall(value)),
            ExpressionInner::ExpressionWrap(value) => {
                Self::try_from((*value.expression.inner).clone())
            }
            ExpressionInner::Var(value) => Err(ConversionError::Var(value)),
            ExpressionInner::Table(value) => Ok(Self::Table(value)),
            ExpressionInner::UnaryExpression {
                operator,
                expression,
            } => Err(ConversionError::UnaryExpression {
                operator,
                expression,
            }),
            ExpressionInner::BinaryExpression {
                left,
                operator,
                right,
            } => Err(ConversionError::BinaryExpression {
                left,
                operator,
                right,
            }),
            ExpressionInner::Cast { cast_to, .. } => Ok((*cast_to.type_value).clone()),
            ExpressionInner::IfExpression {
                if_expression,
                else_if_expressions,
                else_expression,
                ..
            } => Ok(TypeValue::Union {
                left: Arc::new(Self::try_from((*if_expression.inner).clone())?),
                pipe: SingleToken::from(" | "),
                right: Arc::new(TypeValue::Union {
                    left: Arc::new(Self::try_from((*else_expression.inner).clone())?),
                    pipe: SingleToken::from(" | "),
                    right: Arc::new(else_if_to_type(else_if_expressions.to_vec(), 0)?),
                }),
            }),
        }
    }
}
