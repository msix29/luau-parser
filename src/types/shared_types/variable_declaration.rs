use crate::prelude::TypeDefinition;

/// A struct holding data for variable declarations.
#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    /// The name of the variable.
    pub variable_name: String,

    /// The _[type](TypeDefinition)_ of the variable.
    pub variable_type: TypeDefinition,
}
