//! Module holding do blocks.

use crate::prelude::{Ast, SingleToken};

#[derive(Clone, Debug)]
/// A struct representing a do statement
pub struct DoBlock {
    /// The `do` keyword.
    pub do_keyword: SingleToken,

    /// The body of the do block.
    pub body: Ast,

    /// The `end` keyword
    pub end_keyword: SingleToken,
}
