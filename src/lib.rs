//! # Luau Parser
//!
//! A lossless parser for the luau programming language, lossless means that none of the
//! details of the code is lost, it's all stored in the returned syntax tree, and thus, the
//! source code can be printed back from using the _[ast::print](types::Ast::print)_
//! function.
//!
//! # Usage:
//!
//! ```rust
//! let code = "";
//! let uri = "";
//!
//! let mut parser = Parser::new();
//! let ast = parser.parse(code, uri);
//!
//! println!("{:#?}", ast);
//! ```
//!
//! # Note
//!
//! * This parser does not stop parsing when it finds an error
//! * This parser only parses the code into an understandable syntax tree, it does not
//!     garuntee that the code itself is error free. Usage of undefined items will not
//!     produce wrong results.
//! * This parser only works for luau, although for lua versions compatible with luau, it
//! can still be used, for example, lua 5.1, but features limited to a version of lua
//! won't work, for example attributes in lua 5.3.

#![forbid(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::arc_with_non_send_sync)]
#![warn(missing_docs)]

pub mod ast;
mod macros;
pub mod parser;
pub mod types;
mod utils;

/// Loads all needed items for outside crates to use.
pub mod prelude {
    pub use crate::ast::*;
    pub use crate::parser::*;
    pub use crate::types::*;
}
