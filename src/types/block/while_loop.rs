//! While loops.

use luau_lexer::prelude::Token;
use luau_parser_derive::Range;

use crate::types::{Pointer, DoBlock, Expression};

/// A struct representing a while loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WhileLoop {
    /// The "while" keyword.
    pub while_keyword: Token,

    /// The condition of the while loop.
    pub condition: Pointer<Expression>,

    /// The do block.
    pub do_block: DoBlock,
}
