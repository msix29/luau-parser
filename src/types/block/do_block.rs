//! Module holding do blocks.

use luau_lexer::prelude::Token;

use crate::prelude::Ast;

/// A struct representing a do statement
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct DoBlock {
    /// The `do` keyword.
    pub do_keyword: Token,

    /// The body of the do block.
    pub body: Ast,

    /// The `end` keyword
    pub end_keyword: Token,
}
