//! For in struct.

use luau_lexer::prelude::Token;
use std::sync::Arc;

use super::DoBlock;
use crate::prelude::{Expression, List, NormalizedName};

/// A struct representing a for-in loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GenericFor {
    /// The `for` keyword.
    pub for_keyword: Token,

    /// List of names after the `for` keyword.
    pub names: List<NormalizedName>,

    /// The `in` keyword.
    pub in_keyword: Token,

    /// The expressions after.
    pub expressions: List<Arc<Expression>>,

    /// The do block.
    pub do_block: DoBlock,
}
