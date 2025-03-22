//! # Luau Parser
//!
//! A lossless parser for the luau programming language, lossless means that none of the
//! details of the code is lost, it's all stored in the returned syntax tree, and thus, the
//! source code can be printed back from using the [`Cst::print`](types::Cst::print)
//! function.
//!
//! # Usage:
//!
//! ```no_run
//! use luau_parser::parser::LuauParser;
//!
//! let code = r#"local foo = "Hello, World!""#;
//! let uri = ""; // This should be the path of the file being parsed
//!               // (Used with `cache` feature).
//!
//! let mut parser = LuauParser::new(code);
//! let cst = parser.parse(uri);
//!
//! println!("{:#?}", cst);
//! assert!(!cst.block.is_empty());
//! ```
//!
//! # Note
//!
//! * This parser does not stop parsing when it finds an error
//! * This parser only parses the code into an understandable syntax tree, it does not
//!     garuntee that the code itself is error free. Usage of undefined items will not
//!     produce wrong results. Statements with syntax errors in them will not be parsed
//!     though.
//! * This parser only works for luau, although for lua versions compatible with luau, it
//!     can still be used, for example, lua 5.1, but features limited to a version of lua
//!     won't work, for example attributes in lua 5.3.

#![deny(unsafe_code)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::arc_with_non_send_sync)]
// #![warn(missing_docs)]
// #![allow(unused)]
#![warn(clippy::absolute_paths)]

mod r#impl;
// mod display;
mod macros;
pub mod parser;
pub mod types;
mod utils;
pub use luau_lexer;

/// Loads all needed items for outside crates to use.
pub mod prelude {
    pub use crate::parser::*;
    pub use crate::types::*;
    pub use luau_lexer::prelude::*;
}
