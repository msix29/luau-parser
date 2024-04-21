//! The `find_type` function.

use std::sync::Arc;

use crate::prelude::{Ast, Statement, TypeDefinition};

/// Finds a type with a specific name in a specific [`ast`](Ast).
///
/// # Note
///
/// This function will find the **first** occurrence of the type and return it.
pub fn find_type<'a>(ast: &'a Ast, type_name: &'a str) -> Option<Arc<&'a TypeDefinition>> {
    for token in ast.statements.iter() {
        if let Statement::TypeDefinition(type_definition) = &token.0 {
            if type_definition.type_name.word == type_name {
                return Some(Arc::new(type_definition));
            }
        }
    }

    None
}
