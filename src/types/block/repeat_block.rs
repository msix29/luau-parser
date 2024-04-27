//! Repeat blocks.

use std::sync::Arc;

use crate::prelude::{Ast, Expression, Token};

/// A struct representing a repeat block.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RepeatBlock {
    /// The "repeat" keyword.
    pub repeat_keyword: Token,

    /// The body of this repeat block.
    pub body: Ast,

    /// The "until" keyword.
    pub until_keyword: Token,

    /// The condition that will stop this block from running.
    pub condition: Arc<Expression>,
}
