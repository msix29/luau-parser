//! For in struct.

use luau_lexer::prelude::Token;
use luau_parser_derive::Range;

use crate::types::{DoBlock, Expression, List, Name, Pointer};

/// A struct representing a for-in loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GenericFor {
    /// The `for` keyword.
    pub for_keyword: Token,

    /// List of names after the `for` keyword.
    pub names: List<Name>,

    /// The `in` keyword.
    pub in_keyword: Token,

    /// The expressions after.
    pub expressions: List<Pointer<Expression>>,

    /// The do block.
    pub do_block: DoBlock,
}
