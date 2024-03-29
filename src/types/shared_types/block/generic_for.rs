//! For in struct.

use crate::prelude::{Expression, List, NormalizedName, SingleToken};

use super::DoBlock;

/// A struct representing a for-in loop.
///
/// ```lua
/// for i, v in ipairs(t) do
///     print(`{i}: {v}`)
/// end
/// ```
#[derive(Clone, Debug)]
pub struct GenericFor {
    /// The `for` keyword.
    pub for_keyword: SingleToken,

    /// List of names after the `for` keyword.
    pub names: List<NormalizedName>,

    /// The `in` keyword.
    pub in_keyword: SingleToken,

    /// The expressions after.
    pub expressions: List<Expression>,

    /// The do block.
    pub do_block: DoBlock,
}
