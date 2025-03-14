//! While loops.

use luau_lexer::prelude::Token;
use std::sync::Arc;

use crate::prelude::{DoBlock, Expression};

/// A struct representing a while loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct WhileLoop {
    /// The "while" keyword.
    pub while_keyword: Token,

    /// The condition of the while loop.
    pub condition: Arc<Expression>,

    /// The do block.
    pub do_block: DoBlock,
}
