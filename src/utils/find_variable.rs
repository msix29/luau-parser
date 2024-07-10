//! The `find_variable` function.

use std::sync::Arc;

use crate::prelude::{
    Ast, Expression, GlobalFunction, HasRange, HasRawValue, List, LocalAssignment, LocalFunction,
    NormalizedName, Position, Range, Reference, SetExpression, Statement, Token, TypeValue, Var,
    VariableName, CURRENT_URI,
};

/// a
type Variable = (Token, Option<Arc<TypeValue>>, Arc<Expression>);

/// Main function for [`find_variable`] and [`increment_reference`]
fn find_variable_inner<'a>(
    ast: &'a Ast,
    variable_name: &'a str,
    position: Position,
    local_variable_callback: impl Fn(usize, &NormalizedName, &LocalAssignment) -> Option<Variable>,
    set_expression_callback: impl Fn(usize, &VariableName, &SetExpression) -> Option<Variable>,
    local_function_callback: impl Fn(&LocalFunction) -> Option<Variable>,
    global_function_callback: impl Fn(&GlobalFunction) -> Option<Variable>,
) -> Option<Variable> {
    for (statement, _) in ast.statements.iter().rev() {
        match statement {
            Statement::LocalAssignment(local_assignment) => {
                if local_assignment.get_range().end.is_after(&position) {
                    continue;
                }

                for (i, normalized_name) in local_assignment.name_list.iter().enumerate() {
                    if normalized_name.name.word == variable_name {
                        return local_variable_callback(i, normalized_name, local_assignment);
                    }
                }
            }
            Statement::SetExpression(set_expression) => {
                if set_expression.get_range().end.is_after(&position) {
                    continue;
                }

                for (i, var) in set_expression.variables.iter().enumerate() {
                    if let Var::Name(name) = &**var {
                        if name.word == variable_name {
                            return set_expression_callback(i, name, set_expression);
                        }
                    }
                }
            }
            Statement::LocalFunction(local_function) => {
                if local_function.get_range().end.is_after(&position) {
                    continue;
                }

                if local_function.function_name.word == variable_name {
                    return local_function_callback(local_function);
                }
            }
            Statement::GlobalFunction(global_function) => {
                if global_function.get_range().end.is_after(&position) {
                    continue;
                }

                if global_function.function_name.get_raw_value() == variable_name {
                    return global_function_callback(global_function);
                }
            }
            _ => (),
        }
    }

    None
}

/// Crate-level version of [`find_variable`], this has the ability to return `None` even
/// if it found the variable, just to avoid all the `.clone()`ing when the only need for
/// it so to add to the `references` table.
pub(crate) fn increment_reference<'a>(
    ast: &'a Ast,
    variable_name: &'a str,
    position: Position,
) -> Option<Variable> {
    find_variable_inner(
        ast,
        variable_name,
        position,
        |_, normalized_name, _| {
            let _ = normalized_name.references.push(Reference {
                uri: CURRENT_URI.read().unwrap().clone(),
                range: Range::new(position, position),
            });
            None
        },
        |i, variable_name, set_expression| None,
        |local_function| None,
        |global_function| None,
    )
}

/// The body of match arms for [`LocalFunction`]s and [`GlobalFunction`]s.
macro_rules! __function_handler {
    ($variable_name: ident, $name_expression: expr) => {{
        Some((
            $name_expression,
            Some(Arc::new(TypeValue::Function {
                generics: $variable_name.generics.clone(),
                opening_parenthesis: $variable_name.opening_parenthesis.clone(),
                parameters: $variable_name.parameters.clone(),
                closing_parenthesis: $variable_name.closing_parenthesis.clone(),
                arrow: Token::new("->"),
                return_type: $variable_name.returns.clone().unwrap_or_else(|| {
                    Arc::new(TypeValue::Tuple {
                        opening_parenthesis: Token::new("("),
                        types: List::default(),
                        closing_parenthesis: Token::new(")"),
                    })
                }),
            })),
            Arc::new(Expression::Function {
                function_keyword: $variable_name.function_keyword.clone(),
                generics: $variable_name.generics.clone(),
                opening_parenthesis: $variable_name.opening_parenthesis.clone(),
                parameters: $variable_name.parameters.clone(),
                closing_parenthesis: $variable_name.closing_parenthesis.clone(),
                colon: $variable_name.colon.clone(),
                returns: $variable_name.returns.clone(),
                body: $variable_name.body.clone(),
                end_keyword: $variable_name.end_keyword.clone(),
            }),
        ))
    }};
}

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
    position: Position,
) -> Option<Variable> {
    find_variable_inner(
        ast,
        variable_name,
        position,
        |i, normalized_name, local_assignment| {
            if let Some(expression) = local_assignment.expressions.get(i) {
                Some((
                    normalized_name.name.clone(),
                    normalized_name.r#type.clone(),
                    (**expression).clone(),
                ))
            } else {
                Some((
                    normalized_name.name.clone(),
                    normalized_name.r#type.clone(),
                    Arc::new(Expression::Nil(Token::new("nil"))),
                ))
            }
        },
        |i, variable_name, set_expression| {
            if let Some(expression) = set_expression.values.get(i) {
                Some((variable_name.token.clone(), None, (**expression).clone()))
            } else {
                Some((
                    variable_name.token.clone(),
                    None,
                    Arc::new(Expression::Nil(Token::new("nil"))),
                ))
            }
        },
        |local_function| __function_handler!(local_function, local_function.function_name.clone()),
        |global_function| {
            __function_handler!(
                global_function,
                Token::new(&global_function.function_name.get_raw_value())
            )
        },
    )
}
