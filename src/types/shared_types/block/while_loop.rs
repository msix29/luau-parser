//! While loops.

use crate::prelude::{DoBlock, Expression, SingleToken};

/// A struct representing a while loop.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WhileLoop {
    /// The "while" keyword.
    pub while_keyword: SingleToken,

    /// The condition of the while loop.
    pub condition: Expression,

    /// The do block.
    pub do_block: DoBlock,
}
