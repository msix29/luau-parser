use super::{Expression, NormalizedName, SingleToken};

/// A struct holding data for variable declarations.
#[derive(Clone, Debug, Default)]
pub struct VariableDeclaration {
    /// The keyword `local`.
    pub local_token: Option<SingleToken>,

    /// The name of the variable.
    pub variable_name: Box<NormalizedName>,

    /// The `=`.
    pub equal_token: Option<SingleToken>,

    /// The _[expression](Expression)_ of the variable. This may be an empty string if this
    /// value was returned from a function.
    pub variable_value: Box<Expression>,
}
