//! For in struct.

use std::sync::Arc;

use super::DoBlock;
use crate::{
    generate_derives,
    prelude::{Expression, List, NormalizedName, Token},
};

generate_derives! {
    /// A struct representing a for-in loop.
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
}
