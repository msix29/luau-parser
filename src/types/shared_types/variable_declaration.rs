use super::{NormalizedName, SingleToken, Value};

/// A struct holding data for variable declarations.
#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    /// The keyword `local`.
    pub local_token: Option<SingleToken>,

    /// The name of the variable.
    pub variable_name: NormalizedName,

    /// The `=`.
    pub equal_token: Option<SingleToken>,

    /// The _[value](Value)_ of the variable. This may be an empty string if this value
    /// is returned from a function.
    pub variable_value: Value,
}
