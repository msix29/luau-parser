//! The `find_variable` function.

use std::sync::Arc;

use crate::prelude::{
    Ast, Expression, HasRange, Position, SingleToken, Statement, TypeDefinition, Var,
};

type Variable<'a> = Option<(
    &'a SingleToken,
    Option<Arc<TypeDefinition>>,
    Arc<Expression>,
)>;

/// Finds a variable with a specific name in a specific [`ast`](Ast). The
/// [`position`](Position) is needed so that it finds the variable that's before it.
pub fn find_variable<'a>(
    ast: &'a Ast,
    variable_name: &'a str,
    position: &Position,
) -> Variable<'a> {
    for token in ast.statements.iter().rev() {
        if let Statement::LocalAssignment(value) = &token.0 {
            if value.get_range().end.is_after(position) {
                continue;
            }

            for (i, normalized_name) in value.name_list.iter().enumerate() {
                if normalized_name.name.word == variable_name {
                    if let Some(expression) = value.expressions.get(i) {
                        return Some((
                            &normalized_name.name,
                            normalized_name.r#type.clone(),
                            (&**expression).clone(),
                        ));
                    } else {
                        return Some((
                            &normalized_name.name,
                            normalized_name.r#type.clone(),
                            Arc::new(Expression::Nil(SingleToken::new("nil"))),
                        ));
                    }
                }
            }
        } else if let Statement::SetExpression(value) = &token.0 {
            if value.get_range().end.is_after(position) {
                continue;
            }

            for (i, var) in value.variables.iter().enumerate() {
                if let Var::Name(name) = &**var {
                    if name.word == variable_name {
                        if let Some(expression) = value.values.get(i) {
                            return Some((&name, None, (&**expression).clone()));
                        } else {
                            return Some((
                                &name,
                                None,
                                Arc::new(Expression::Nil(SingleToken::new("nil"))),
                            ));
                        }
                    }
                }
            }
        }
    }

    None
}
