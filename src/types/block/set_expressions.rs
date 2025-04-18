//! Set expressions.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{Expression, List, Pointer, Var};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A struct representing a set expression.
pub struct SetExpression {
    /// The variables whom values are being set.
    pub variables: List<Var>,

    /// The `=` character.
    pub equal: Token,

    /// The values that are being set, in the same order as variables, the length of
    /// these values may be more or less than the variables list.
    pub values: List<Pointer<Expression>>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// A struct representing a set expression.
pub struct CompoundSetExpression {
    /// The variables whom values are being set.
    pub variable: Var,

    /// The compound operation like `+=`, `//=`, etc.
    pub operation: Token,

    /// The values that are being set, in the same order as variables, the length of
    /// these values may be more or less than the variables list.
    pub value: Pointer<Expression>,
}
