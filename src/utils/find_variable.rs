//! The `find_variable` function.

// use crate::{Ast, Expression, HasRange, NormalizedName, Statement};

use std::sync::Arc;

use crate::prelude::{
    Ast, Expression, HasRange, Position, SingleToken, Statement, TypeDefinition, Var,
};

/// Finds a variable in a specific name in a specific [`ast`](Ast). The
/// [`position`](Position) is needed so that it finds the variable that's before it.
pub fn find_variable<'a>(
    ast: &'a Ast,
    variable_name: &'a str,
    position: &Position,
) -> Option<(&'a SingleToken, Option<Arc<TypeDefinition>>, &'a Expression)> {
    for token in ast.statements.iter().rev() {
        if let Statement::LocalAssignment(value) = &token.0 {
            if value.get_range().end.is_after(position) {
                continue;
            }

            for (i, normalized_name) in value.name_list.items.iter().enumerate() {
                if normalized_name.name.word == variable_name {
                    return value.expressions.items.get(i).map(|expression| {
                        (
                            &normalized_name.name,
                            normalized_name.r#type.clone(),
                            &**expression,
                        )
                    });
                }
            }
        } else if let Statement::SetExpression(value) = &token.0 {
            if value.get_range().end.is_after(position) {
                continue;
            }

            for (i, var) in value.variables.items.iter().enumerate() {
                if let Var::Name(name) = &**var {
                    if name.word == variable_name {
                        return value
                            .values
                            .items
                            .get(i)
                            .map(|expression| (name, None, &**expression));
                    }
                }
            }
        }
    }

    None
}
