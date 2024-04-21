//! For in struct.

use std::sync::Arc;

use crate::prelude::{Expression, List, NormalizedName, SingleToken};

use super::DoBlock;

/// A struct representing a for-in loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GenericFor {
    /// The `for` keyword.
    pub for_keyword: SingleToken,

    /// List of names after the `for` keyword.
    pub names: List<NormalizedName>,

    /// The `in` keyword.
    pub in_keyword: SingleToken,

    /// The expressions after.
    pub expressions: List<Arc<Expression>>,

    /// The do block.
    pub do_block: DoBlock,
}
