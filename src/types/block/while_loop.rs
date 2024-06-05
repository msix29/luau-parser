//! While loops.

use std::sync::Arc;

use crate::{
    generate_derives,
    prelude::{DoBlock, Expression, Token},
};

generate_derives! {
    /// A struct representing a while loop.
    pub struct WhileLoop {
        /// The "while" keyword.
        pub while_keyword: Token,

        /// The condition of the while loop.
        pub condition: Arc<Expression>,

        /// The do block.
        pub do_block: DoBlock,
    }
}
