use crate::prelude::TypeDefinition;

#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    pub variable_name: String,
    pub variable_type: TypeDefinition,
}
