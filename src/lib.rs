#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]

#[macro_use]
mod macros;

mod r#impl;
pub mod parser;
pub mod types;
mod utils;
pub use luau_lexer;

/// Loads all needed items for outside crates to use.
pub mod prelude {
    pub use crate::parser::*;
    pub use crate::types::*;
    pub use luau_lexer::prelude::{Comment, *};
}
