//! Numerical for loop struct

use luau_lexer::prelude::Token;
use std::sync::Arc;

use super::DoBlock;
use crate::prelude::{Expression, Name};

/// A struct representing a numerical for loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NumericalFor {
    /// The `for` keyword.
    pub for_keyword: Token,

    /// The name afer the `for` keyword.
    pub variable: Name,

    /// The `=` character.
    pub equal_keyword: Token,

    /// The start of the loop.
    pub start: Arc<Expression>,

    /// The comma after the start.
    pub start_comma: Token,

    /// The end of the loop.
    pub end: Arc<Expression>,

    /// The comma after the end of the loop.
    pub end_comma: Option<Token>,

    /// The optional step of the loop.
    pub step: Option<Arc<Expression>>,

    /// The do block at the end
    pub do_block: DoBlock,
}
