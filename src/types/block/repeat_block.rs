//! Repeat blocks.

use std::sync::Arc;

use crate::{
    generate_derives,
    prelude::{Ast, Expression, Token},
};

generate_derives! {
    /// A struct representing a repeat block.
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
}
