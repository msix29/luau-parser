use crate::prelude::TypeDefinition;

use super::Value;

/// A struct holding data for variable declarations.
#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    /// The name of the variable.
    pub variable_name: String,

    /// The _[type](TypeDefinition)_ of the variable.
    pub variable_type: TypeDefinition,

    /// The _[value](Value)_ of the variable. This may be an empty string if this value
    /// is returned from a function.
    pub variable_value: Value,
}
