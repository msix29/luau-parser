//! Set expressions.

use super::{Expression, List, SingleToken, Var};

/// A struct representing a set expression.
#[derive(Clone, Debug)]
pub struct SetExpression {
    /// The variables whome values are being set.
    pub variables: List<Var>,

    /// The `=` character.
    pub equal: SingleToken,

    /// The values that are being set, in the same order as variables, the length of
    /// these values may be more or less than the variables list.
    pub values: List<Expression>,
}
