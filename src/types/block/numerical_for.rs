//! Numerical for loop struct

use crate::prelude::{Expression, NormalizedName, SingleToken};

use super::DoBlock;

/// A struct representing a numerical for loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NumericalFor {
    /// The `for` keyword.
    pub for_keyword: SingleToken,

    /// The name afer the `for` keyword.
    pub variable: NormalizedName,

    /// The `=` character.
    pub equal_keyword: SingleToken,

    /// The start of the loop.
    pub start: Expression,

    /// The comma after the start.
    pub start_comma: SingleToken,

    /// The end of the loop.
    pub end: Expression,

    /// The comma after the end of the loop.
    pub end_comma: Option<SingleToken>,

    /// The optional step of the loop.
    pub step: Option<Expression>,

    /// The do block at the end
    pub do_block: DoBlock,
}
