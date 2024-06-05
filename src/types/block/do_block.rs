//! Module holding do blocks.

use crate::{
    generate_derives,
    prelude::{Ast, Token},
};

generate_derives! {
    /// A struct representing a do statement
    pub struct DoBlock {
        /// The `do` keyword.
        pub do_keyword: Token,

        /// The body of the do block.
        pub body: Ast,

        /// The `end` keyword
        pub end_keyword: Token,
    }
}
