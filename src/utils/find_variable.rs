//! The `find_variable` function.

use std::sync::Arc;

use crate::prelude::{
    Ast, Expression, HasRange, HasRawValue, List, Position, Statement, Token, TypeValue, Var,
};

type Variable = (Token, Option<Arc<TypeValue>>, Arc<Expression>);

/// Finds a variable with a specific name in a specific [`ast`](Ast). The
/// [`position`](Position) is needed so that it finds the variable that's before it.
///
/// # Note
///
/// This function has a lot of `clone()`ing, but it should be cheap as it's mostly for
/// `Arc<T>`s.
pub fn find_variable<'a>(
    ast: &'a Ast,
    variable_name: &'a str,
    position: &Position,
) -> Option<Variable> {
    for (statement, _) in ast.statements.iter().rev() {
        match statement {
            Statement::LocalAssignment(local_assignment) => {
                if local_assignment.get_range().end.is_after(position) {
                    continue;
                }

                for (i, normalized_name) in local_assignment.name_list.iter().enumerate() {
                    if normalized_name.name.word == variable_name {
                        if let Some(expression) = local_assignment.expressions.get(i) {
                            return Some((
                                normalized_name.name.clone(),
                                normalized_name.r#type.clone(),
                                (&**expression).clone(),
                            ));
                        } else {
                            return Some((
                                normalized_name.name.clone(),
                                normalized_name.r#type.clone(),
                                Arc::new(Expression::Nil(Token::new("nil"))),
                            ));
                        }
                    }
                }
            }
            Statement::SetExpression(set_expression) => {
                if set_expression.get_range().end.is_after(position) {
                    continue;
                }

                for (i, var) in set_expression.variables.iter().enumerate() {
                    if let Var::Name(name) = &**var {
                        if name.word == variable_name {
                            if let Some(expression) = set_expression.values.get(i) {
                                return Some((name.token.clone(), None, (&**expression).clone()));
                            } else {
                                return Some((
                                    name.token.clone(),
                                    None,
                                    Arc::new(Expression::Nil(Token::new("nil"))),
                                ));
                            }
                        }
                    }
                }
            }
            Statement::LocalFunction(local_function) => {
                if local_function.get_range().end.is_after(position) {
                    continue;
                }

                if local_function.function_name.word == variable_name {
                    return Some((
                        local_function.function_name.clone(),
                        Some(Arc::new(TypeValue::Function {
                            generics: local_function.generics.clone(),
                            opening_parenthesis: local_function.opening_parenthesis.clone(),
                            parameters: local_function.parameters.clone(),
                            closing_parenthesis: local_function.closing_parenthesis.clone(),
                            arrow: Token::new("->"),
                            return_type: local_function.returns.clone().unwrap_or_else(|| {
                                Arc::new(TypeValue::Tuple {
                                    opening_parenthesis: Token::new("("),
                                    types: List::default(),
                                    closing_parenthesis: Token::new(")"),
                                })
                            }),
                        })),
                        Arc::new(Expression::Function {
                            function_keyword: local_function.function_keyword.clone(),
                            generics: local_function.generics.clone(),
                            opening_parenthesis: local_function.opening_parenthesis.clone(),
                            parameters: local_function.parameters.clone(),
                            closing_parenthesis: local_function.closing_parenthesis.clone(),
                            colon: local_function.colon.clone(),
                            returns: local_function.returns.clone(),
                            body: local_function.body.clone(),
                            end_keyword: local_function.end_keyword.clone(),
                        }),
                    ));
                }
            }
            Statement::GlobalFunction(local_function) => {
                if local_function.get_range().end.is_after(position) {
                    continue;
                }

                if local_function.function_name.get_raw_value() == variable_name {
                    return Some((
                        Token::new(&local_function.function_name.get_raw_value()),
                        Some(Arc::new(TypeValue::Function {
                            generics: local_function.generics.clone(),
                            opening_parenthesis: local_function.opening_parenthesis.clone(),
                            parameters: local_function.parameters.clone(),
                            closing_parenthesis: local_function.closing_parenthesis.clone(),
                            arrow: Token::new("->"),
                            return_type: local_function.returns.clone().unwrap_or_else(|| {
                                Arc::new(TypeValue::Tuple {
                                    opening_parenthesis: Token::new("("),
                                    types: List::default(),
                                    closing_parenthesis: Token::new(")"),
                                })
                            }),
                        })),
                        Arc::new(Expression::Function {
                            function_keyword: local_function.function_keyword.clone(),
                            generics: local_function.generics.clone(),
                            opening_parenthesis: local_function.opening_parenthesis.clone(),
                            parameters: local_function.parameters.clone(),
                            closing_parenthesis: local_function.closing_parenthesis.clone(),
                            colon: local_function.colon.clone(),
                            returns: local_function.returns.clone(),
                            body: local_function.body.clone(),
                            end_keyword: local_function.end_keyword.clone(),
                        }),
                    ));
                }
            }
            _ => (),
        }
    }

    None
}
