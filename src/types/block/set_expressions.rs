//! Set expressions.

use luau_lexer::prelude::Token;
use std::sync::Arc;

use crate::types::{Expression, List, Var};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A struct representing a set expression.
pub struct SetExpression {
    /// The variables whome values are being set.
    pub variables: List<Var>,

    /// The `=` character.
    pub equal: Token,

    /// The values that are being set, in the same order as variables, the length of
    /// these values may be more or less than the variables list.
    pub values: List<Arc<Expression>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A struct representing a set expression.
pub struct CompoundSetExpression {
    /// The variables whome values are being set.
    pub variable: Var,

    /// The compound operation like `+=`, `//=`, etc.
    pub operation: Token,

    /// The values that are being set, in the same order as variables, the length of
    /// these values may be more or less than the variables list.
    pub value: Arc<Expression>,
}
