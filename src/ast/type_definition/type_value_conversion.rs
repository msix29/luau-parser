//! Adds conversion from expressions to types.

use std::sync::Arc;

use crate::types::{ConversionError, ElseIfExpression, Expression, List, Token, TypeValue};

/// Recursively turn else if expressions to a type value.
fn else_if_to_type(
    else_if_expressions: Vec<ElseIfExpression>,
    i: usize,
) -> Result<TypeValue, ConversionError> {
    if else_if_expressions.get(i + 1).is_some() {
        Ok(TypeValue::Union {
            left: Arc::new(TypeValue::try_from_expression(
                (*else_if_expressions.get(i).unwrap().expression).clone(),
            )?),
            pipe: Token::from(" | "),
            right: Arc::new(else_if_to_type(else_if_expressions, i + 1)?),
        })
    } else {
        TypeValue::try_from_expression((*else_if_expressions.get(i).unwrap().expression).clone())
    }
}

impl TypeValue {
    /// Tries creating a [`TypeValue`] from an [`Expression`].
    pub fn try_from_expression(expression: Expression) -> Result<Self, ConversionError> {
        match expression {
            Expression::ERROR => Ok(Self::ERROR),
            Expression::Nil(value) => Ok(Self::Basic(value)),
            Expression::Boolean(word) => Ok(Self::Boolean(word)),
            Expression::Number(_) => Ok(Self::Basic(Token::new("number"))),
            Expression::String(value) => Ok(Self::String(value)),
            Expression::Function {
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
                arrow: Token::new("->"),
                return_type: returns.unwrap_or(Arc::new(Self::Tuple {
                    opening_parenthesis: Token::from("("),
                    types: List::default(),
                    closing_parenthesis: Token::from(")"),
                })),
            }),
            Expression::FunctionCall(value) => Err(ConversionError::FunctionCall(value)),
            Expression::ExpressionWrap(value) => {
                Self::try_from_expression((*value.expression).clone())
            }
            Expression::Var(value) => Err(ConversionError::Var(value)),
            Expression::Table(value) => Ok(Self::Table(value)),
            Expression::UnaryExpression {
                operator,
                expression,
            } => Err(ConversionError::UnaryExpression {
                operator,
                expression,
            }),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => Err(ConversionError::BinaryExpression {
                left,
                operator,
                right,
            }),
            Expression::Cast { cast_to, .. } => Ok((*cast_to.type_value).clone()),
            Expression::IfExpression {
                if_expression,
                else_if_expressions,
                else_expression,
                ..
            } => Ok(Self::Union {
                left: Arc::new(Self::try_from_expression((*if_expression).clone())?),
                pipe: Token::from(" | "),
                right: Arc::new(Self::Union {
                    left: Arc::new(Self::try_from_expression((*else_expression).clone())?),
                    pipe: Token::from(" | "),
                    right: Arc::new(else_if_to_type(else_if_expressions.to_vec(), 0)?),
                }),
            }),
        }
    }
}
